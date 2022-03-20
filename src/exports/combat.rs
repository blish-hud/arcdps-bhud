use crate::pubsub::dispatch;
use arcdps_bindings::{cbtevent, Ag, AgOwned};
use smol::Task;

use crate::protos::eventdata::{CombatEvent, CombatMessage, Event, Actor, CombatType};


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

    combatmessage.skillname  = skillname.unwrap_or("").to_string();
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
    proto.self_ = actor.self_;
    proto.id = actor.id as u32;
    proto.prof = actor.prof;
    proto.elite = actor.elite;
    
    proto
}

fn get_ev_proto(ev: &cbtevent) -> CombatEvent {
    let mut proto = CombatEvent::new();
    proto.time = ev.time;
    proto.src_agent = ev.src_agent as u32;
    proto.dst_agent = ev.dst_agent as u32;
    proto.value = ev.value;
    proto.buff_dmg = ev.buff_dmg;
    proto.overstack_value = ev.overstack_value;
    proto.skillid = ev.skillid;
    proto.src_instid = ev.src_instid as u32;
    proto.dst_instid = ev.dst_instid as u32;
    proto.src_master_instid =  ev.src_master_instid as u32;
    proto.iff =  ev.iff as u32;
    proto.buff =  ev.buff as u32;
    proto.result =  ev.result as u32;
    proto.is_activation =  ev.is_activation as u32;
    proto.is_buffremove =  ev.is_buffremove as u32;
    proto.is_ninety =  ev.is_ninety as u32;
    proto.is_fifty =  ev.is_fifty as u32;
    proto.is_moving =  ev.is_moving as u32;
    proto.is_statechange =  ev.is_statechange as u32;
    proto.is_flanking =  ev.is_flanking as u32;
    proto.is_shields =  ev.is_shields as u32;
    proto.is_offcycle =  ev.is_offcycle as u32;
    proto.pad61 =  ev.pad61 as u32;
    proto.pad62 =  ev.pad62 as u32;
    proto.pad63 =  ev.pad63 as u32;
    proto.pad64 =  ev.pad64 as u32;

    proto
}