use crate::pubsub::dispatch;
use arcdps_bindings::{cbtevent, Ag, AgOwned};
use smol::Task;

use crate::protos::eventdata::{
    Actor, Affinity, BuffRemoveType, CombatActivation, CombatEvent, CombatMessage, CombatResult,
    CombatType, Event, EventType, StateChange, WeaponSet, BuffDamageResult
};
use protobuf::ProtobufEnum;

pub fn cbt(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
) {
    spawn_cbt(ev, src, dst, skillname, id, revision, 2);
}

pub fn cbt_local(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
) {
    spawn_cbt(ev, src, dst, skillname, id, revision, 3);
}

fn spawn_cbt(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
    indicator: u8,
) {
    Task::spawn(cbt_with_type(
        ev.copied(),
        src.map(|x| (*x).into()),
        dst.map(|x| (*x).into()),
        skillname,
        id,
        revision,
        indicator,
    ))
    .detach();
}

async fn cbt_with_type(
    ev: Option<cbtevent>,
    src: Option<AgOwned>,
    dst: Option<AgOwned>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
    indicator: u8,
) {
    let mut event = Event::new();
    let mut combatmessage = CombatMessage::new();
    if ev.is_some() {
        combatmessage.set_combat_event(get_ev_proto(&ev.unwrap()));
    }
    if indicator == 2 {
        combatmessage.set_combat_type(CombatType::Area);
    } else {
        combatmessage.set_combat_type(CombatType::Local);
    }
    if src.is_some() {
        combatmessage.set_src_actor(get_actor_proto(&src.unwrap()));
    }

    if dst.is_some() {
        combatmessage.set_dst_actor(get_actor_proto(&dst.unwrap()));
    }

    combatmessage.skillname = skillname.unwrap_or("").to_string();
    combatmessage.id = id;
    combatmessage.revision = revision;

    event.set_combat_message(combatmessage);
    //message.push(indicator); // indicator for local/area combat message
    //add_bytes(&mut message, ev, src, dst, skillname, id, revision);
    dispatch(protobuf::Message::write_to_bytes(&event).unwrap()).await;
}

fn get_actor_proto(actor: &AgOwned) -> Actor {
    let mut proto = Actor::new();
    if actor.name.is_some() {
        proto.name = actor.name.as_ref().unwrap().to_string();
    };
    proto.is_player = actor.self_ == 1;
    proto.id = actor.id as u32;
    proto.Profession = actor.prof;
    if actor.elite != 0xffffffff {
        proto.EliteSpec = actor.elite;
    }
    proto
}

// Reference: https://www.deltaconnected.com/arcdps/evtc/README.txt
fn get_ev_proto(ev: &cbtevent) -> CombatEvent {
    let mut proto = CombatEvent::new();
    proto.time = ev.time;

    proto.shield_damage = ev.overstack_value;
    proto.skillid = ev.skillid;
    proto.src_instid = ev.src_instid as u32;
    proto.dst_instid = ev.dst_instid as u32;
    proto.src_master_instid = ev.src_master_instid as u32;

    proto.friend_or_foe = Affinity::from_i32((ev.iff+1) as i32).unwrap_or(Affinity::AF_Unparsed);
    //proto.buff =  BuffType::from_i32(ev.buff as i32).unwrap_or(BuffType::BT_UNPARSED);
    proto.activation = CombatActivation::from_i32(ev.is_activation as i32)
        .unwrap_or(CombatActivation::CA_UNPARSED);
    proto.buffremove =
        BuffRemoveType::from_i32(ev.is_buffremove as i32).unwrap_or(BuffRemoveType::BR_Unparsed);
    proto.src_hp_over_90 = ev.is_ninety > 0;
    proto.dst_hp_over_50 = ev.is_fifty > 0;
    proto.src_is_moving = ev.is_moving > 0;
    proto.statechange =
        StateChange::from_i32(ev.is_statechange as i32).unwrap_or(StateChange::SC_UNPARSED);
    proto.is_flanking = ev.is_flanking > 0;
    proto.is_offcycle = ev.is_offcycle as u32;
    proto.pad61 = ev.pad61 as u32;
    proto.pad62 = ev.pad62 as u32;
    proto.pad63 = ev.pad63 as u32;
    proto.pad64 = ev.pad64 as u32;

    proto.EventType = EventType::StateChanged;

    match proto.statechange {
        // log start / end
        //  value = server unix timestamp **uint32**.
        //  buff_dmg = local unix timestamp.
        //  src_agent = 0x637261 (arcdps id) if evtc, species id if realtime
        StateChange::LOGSTART | StateChange::LOGEND => {}
        // src_agent swapped weapon set.
        //  dst_agent = current set id (0/1 water, 4/5 land)
        StateChange::WEAPSWAP => {
            proto.weapon_set =
                WeaponSet::from_i32((ev.dst_agent+1) as i32).unwrap_or(WeaponSet::WS_Unparsed);
        }
        // these are the wiggly boxes that you get
        //  src_agent is self,
        //  dst_agent is reward id,
        //  value is reward type.
        StateChange::REWARD => {
            proto.reward_id = ev.dst_agent as u32;
            proto.reward_type = ev.value as u32;
        }
        // src_agent change, dst_agent new team id
        StateChange::TEAMCHANGE => {
            proto.new_team_id = ev.dst_agent as u32;
        }
        // src_agent is agent with buff,
        // value is the duration to reset to (also marks inactive),
        // pad61- is the stackid
        StateChange::STACKRESET => {
            proto.buff_duration = ev.value as i32;
            proto.buff_stackid = ev.pad61 as u32;
        }
        // src_agent is agent,
        // dst_agent through buff_dmg is 16 byte guid (client form, needs minor rearrange for api form)
        StateChange::GUILD => {}
        // src_agent is agent, value is the id (volatile, game build dependent) of the tag
        StateChange::TAG => {
            proto.tag_id = ev.value as u32;
        }

        _ => {
            proto.EventType = EventType::Activation;
            match proto.activation {
                CombatActivation::START => {}
                CombatActivation::CANCEL_FIRE | CombatActivation::CANCEL_CANCEL => {
                    proto.time_in_animation = ev.value as u32;
                    proto.time_in_animation_scaled = ev.buff_dmg as u32;
                }
                CombatActivation::RESET | CombatActivation::QUICKNESS_UNUSED => {
                    proto.time_in_animation = ev.value as u32;
                    proto.time_in_animation_scaled = ev.buff_dmg as u32;
                }
                _ => {
                    // This seems to always be 1
                    // proto.buff_type = BuffType::from_i32(ev.buff as i32).unwrap_or(BuffType::BT_UNPARSED);
                    proto.EventType = EventType::BuffRemoved;
                    match proto.buffremove {
                        BuffRemoveType::All | BuffRemoveType::Single | BuffRemoveType::Manual  => {
                            proto.buff_duration = ev.value as i32;
                            proto.buff_stacks_removed = ev.result as i32;

                        }
                        _ => {
                            if ev.buff > 0 && ev.value > 0 && ev.buff_dmg == 0 {
                                proto.EventType = EventType::BuffApplied;
                                proto.buff_duration = ev.value as i32;
                                proto.buff_added_active = ev.is_shields > 0;
                                // if is_offcycle is zero, overstack_value will be duration of the stack that is expected to be removed.
                                // if is_offcycle is non-zero, overstack_value will be the new duration of the stack, value will be duration change.
                            } else {
                                if ev.buff_dmg > 0 && ev.value == 0 {
                                    proto.EventType = EventType::BuffDamage;
                                    proto.damage = ev.buff_dmg;
                                    proto.buff_damage_result = BuffDamageResult::from_i32((ev.result+1) as i32)
                                        .unwrap_or(BuffDamageResult::BD_Unparsed);
                                } else {
                                    proto.EventType = EventType::DirectDamage;
                                    proto.damage = ev.value;
                                    proto.target_downed = ev.is_offcycle == 1;
                                    proto.result = CombatResult::from_i32(ev.result as i32)
                                        .unwrap_or(CombatResult::CR_Unparsed);
                                }
                            }
                        }
                }},
            }
        }
    }
    return proto;
}
