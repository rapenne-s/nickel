use crate::{
    eval::{
        Closure,
        cache::{Cache, CacheIndex},
    },
    identifier::  
}

pub struct DumbCacheEntry {
    closure: Closure,
    ident_kind: IdentKind,
    bty: BindingType,
}

pub struct DumbCache {
    cache: Vec<DumbCacheEntry>,
}

pub type DumbCacheIndex = usize;

impl Cache for DumbCache {
    type UpdateIndex = usize;

    fn get(&self, idx: CacheIndex) -> Closure {
        idx.get_owned()
    }

    fn get_update_index(
        &mut self,
        idx: &mut CacheIndex,
    ) -> Result<Option<Self::UpdateIndex>, BlackholedError> {
        if idx.state() != ThunkState::Evaluated {
            if idx.should_update() {
                idx.mk_update_frame().map(Some)
            }
            // If the thunk isn't to be updated, directly set the evaluated flag.
            else {
                idx.set_evaluated();
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn add(&mut self, clos: Closure, kind: IdentKind, bty: BindingType) -> CacheIndex {
        match bty {
            BindingType::Normal => Thunk::new(clos, kind),
            BindingType::Revertible(deps) => Thunk::new_rev(clos, kind, deps),
        }
    }

    fn patch<F: FnOnce(&mut Closure)>(&mut self, mut idx: CacheIndex, f: F) {
        f(&mut idx.borrow_mut());
    }

    fn get_then<T, F: FnOnce(&Closure) -> T>(&self, idx: CacheIndex, f: F) -> T {
        f(&idx.borrow())
    }

    fn update(&mut self, clos: Closure, uidx: Self::UpdateIndex) {
        uidx.update(clos);
    }

    fn new() -> Self {
        CBNCache {}
    }

    fn reset_index_state(&self, idx: &mut Self::UpdateIndex) {
        idx.reset_state();
    }

    fn map_at_index<F: FnMut(&Closure) -> Closure>(
        &mut self,
        idx: &CacheIndex,
        f: F,
    ) -> CacheIndex {
        idx.map(f)
    }

    fn build_cached(&mut self, idx: &mut CacheIndex, rec_env: &[(Ident, CacheIndex)]) {
        idx.build_cached(rec_env)
    }
}
}
