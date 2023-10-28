use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdResult, QuerierWrapper};
use cw_storage_plus::Map;
use crate::query::{NAMES_KEY, ADDR_KEY};

/// Cw5Contract is a wrapper around Addr that provides helpers
/// for working with metadata. Specially names, oriented to final users of dapps.
///
/// If you wish to persist this, convert to Cw1CanonicalContract via .canonical()
#[cw_serde]
pub struct Cw5Contract(pub Addr);

impl Cw5Contract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn look_up(&self, querier: &QuerierWrapper, name: String) -> StdResult<LookUpResponse> {
        //let addr = ADDR_RESOLVER.may_load(self, name.to_string())?;
        let addr = Map::new(ADDR_KEY).query(querier, self.addr(), name)?;        
        Ok(LookUpResponse { addr })
    }

    pub fn reverse_look_up(&self, querier: &QuerierWrapper, addr: String) -> StdResult<ReverseLookUpResponse> {  // maybe <T: Into<Addr>>
        //let addr = deps.api.addr_validate(&addr)?;
        //let name = NAMES_RESOLVER.may_load(self, addr)?;
        let name = Map::new(NAMES_KEY).query(querier, self.addr(), addr)?;
        Ok(ReverseLookUpResponse { name })
    }

    // metadata too?
}

#[cw_serde]
pub struct LookUpResponse {
    pub addr: Option<String>,
}

#[cw_serde]
pub struct ReverseLookUpResponse {
    pub name: Option<String>,
}
