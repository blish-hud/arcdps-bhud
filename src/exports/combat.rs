use crate::pubsub::dispatch;
use arcdps::{CombatEvent, Agent, AgentOwned};
use smol::Task;

pub fn cbt(
    ev: Option<&CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
) {
    spawn_cbt(ev, src, dst, skillname, id, revision, 2);
}

pub fn cbt_local(
    ev: Option<&CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
) {
    spawn_cbt(ev, src, dst, skillname, id, revision, 3);
}

fn spawn_cbt(
    ev: Option<&CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
    indicator: u8,
) {
    Task::spawn(cbt_with_type(
        ev.copied(),
        src.map(|x| x.into()),
        dst.map(|x| x.into()),
        skillname,
        id,
        revision,
        indicator,
    ))
    .detach();
}

async fn cbt_with_type(
    ev: Option<CombatEvent>,
    src: Option<AgentOwned>,
    dst: Option<AgentOwned>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
    indicator: u8,
) {
    let mut message = Vec::new();
    message.push(indicator); // indicator for local/area combat message
    add_bytes(&mut message, ev, src, dst, skillname, id, revision);
    dispatch(message).await;
}

fn add_bytes(
    message: &mut Vec<u8>,
    ev: Option<CombatEvent>,
    src: Option<AgentOwned>,
    dst: Option<AgentOwned>,
    skillname: Option<&str>,
    id: u64,
    revision: u64,
) {
    let mut messages = 0;
    if let Some(ev) = ev {
        messages |= 1;
        let mut bytes = get_ev_bytes(&ev);
        message.append(&mut bytes);
    };
    if let Some(ag) = src {
        messages |= 1 << 1;
        let mut bytes = get_ag_bytes(&ag);
        message.append(&mut bytes);
    };
    if let Some(ag) = dst {
        messages |= 1 << 2;
        let mut bytes = get_ag_bytes(&ag);
        message.append(&mut bytes);
    };
    if let Some(name) = skillname {
        messages |= 1 << 3;
        let bytes = name.as_bytes();
        let mut bytes = [&bytes.len().to_le_bytes(), bytes].concat();
        message.append(&mut bytes);
    };
    message.insert(1, messages);
    message.append(&mut id.to_le_bytes().to_vec());
    message.append(&mut revision.to_le_bytes().to_vec());
}

fn get_ev_bytes(ev: &CombatEvent) -> Vec<u8> {
    ev.time
        .to_le_bytes()
        .iter()
        .chain(ev.src_agent.to_le_bytes().iter())
        .chain(ev.dst_agent.to_le_bytes().iter())
        .chain(ev.value.to_le_bytes().iter())
        .chain(ev.buff_dmg.to_le_bytes().iter())
        .chain(ev.overstack_value.to_le_bytes().iter())
        .chain(ev.skill_id.to_le_bytes().iter())
        .chain(ev.src_instance_id.to_le_bytes().iter())
        .chain(ev.dst_instance_id.to_le_bytes().iter())
        .chain(ev.src_master_instance_id.to_le_bytes().iter())
        .chain(ev.dst_master_instance_id.to_le_bytes().iter())
        .chain(ev.iff.to_le_bytes().iter())
        .chain(ev.buff.to_le_bytes().iter())
        .chain(ev.result.to_le_bytes().iter())
        .chain(ev.is_activation.to_le_bytes().iter())
        .chain(ev.is_buff_remove.to_le_bytes().iter())
        .chain(ev.is_ninety.to_le_bytes().iter())
        .chain(ev.is_fifty.to_le_bytes().iter())
        .chain(ev.is_moving.to_le_bytes().iter())
        .chain(ev.is_statechange.to_le_bytes().iter())
        .chain(ev.is_flanking.to_le_bytes().iter())
        .chain(ev.is_shields.to_le_bytes().iter())
        .chain(ev.is_off_cycle.to_le_bytes().iter())
        .chain(ev.pad61.to_le_bytes().iter())
        .chain(ev.pad62.to_le_bytes().iter())
        .chain(ev.pad63.to_le_bytes().iter())
        .chain(ev.pad64.to_le_bytes().iter())
        .cloned()
        .collect::<Vec<u8>>()
}

fn get_ag_bytes(ag: &AgentOwned) -> Vec<u8> {
    let (string_length, name_bytes) = if let Some(name) = &ag.name {
        let bytes = name.as_bytes();
        (bytes.len(), Some(bytes))
    } else {
        (0, None)
    };
    if let Some(name_bytes) = name_bytes {
        string_length
            .to_le_bytes()
            .iter()
            .chain(name_bytes.iter())
            .chain(ag.id.to_le_bytes().iter())
            .chain(ag.prof.to_le_bytes().iter())
            .chain(ag.elite.to_le_bytes().iter())
            .chain(ag.self_.to_le_bytes().iter())
            .chain(ag.team.to_le_bytes().iter())
            .cloned()
            .collect()
    } else {
        string_length
            .to_le_bytes()
            .iter()
            .chain(ag.id.to_le_bytes().iter())
            .chain(ag.prof.to_le_bytes().iter())
            .chain(ag.elite.to_le_bytes().iter())
            .chain(ag.self_.to_le_bytes().iter())
            .chain(ag.team.to_le_bytes().iter())
            .cloned()
            .collect()
    }
}
