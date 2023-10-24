use cosmwasm_std::Addr;
use cw4::{
    MEMBERS_CHANGELOG, MEMBERS_CHECKPOINTS, MEMBERS_KEY, TOTAL_KEY, TOTAL_KEY_CHANGELOG,
    TOTAL_KEY_CHECKPOINTS,
};
use cw_controllers::{Admin, Hooks};
use cw_storage_plus::{SnapshotItem, SnapshotMap, Strategy, Map};

pub const ADMIN: Admin = Admin::new("admin");
pub const HOOKS: Hooks = Hooks::new("cw4-hooks");


pub const TOTAL: SnapshotItem<u64> = SnapshotItem::new(
    TOTAL_KEY,
    TOTAL_KEY_CHECKPOINTS,
    TOTAL_KEY_CHANGELOG,
    Strategy::EveryBlock,
);

pub const MEMBERS: SnapshotMap<&Addr, u64> = SnapshotMap::new(
    MEMBERS_KEY,
    MEMBERS_CHECKPOINTS,
    MEMBERS_CHANGELOG,
    Strategy::EveryBlock,
);

pub const NAMES_KEY: &str = "names";
pub const NAMES_RESOLVER: Map<&Addr, String> = Map::new(
    NAMES_KEY,
);

pub const ADDR_KEY: &str = "address";

pub const ADDR_RESOLVER: Map<String, String> = Map::new(
    ADDR_KEY,
);