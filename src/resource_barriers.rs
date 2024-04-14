use super::*;
use std::mem::ManuallyDrop;
use windows::Win32::Graphics::Direct3D12::*;

pub trait ResourceBarrier {
    fn as_raw(&self) -> &D3D12_RESOURCE_BARRIER;
}

#[derive(Clone)]
pub struct TransitionBarrier(Option<D3D12_RESOURCE_BARRIER>);

impl TransitionBarrier {
    #[inline]
    pub fn new(
        resource: &Resource,
        subresource: u32,
        state_before: D3D12_RESOURCE_STATES,
        state_after: D3D12_RESOURCE_STATES,
        flags: D3D12_RESOURCE_BARRIER_FLAGS,
    ) -> Self {
        Self(Some(D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: flags,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: ManuallyDrop::new(Some(resource.handle().clone())),
                    Subresource: subresource,
                    StateBefore: state_before,
                    StateAfter: state_after,
                }),
            },
        }))
    }
}

impl Drop for TransitionBarrier {
    fn drop(&mut self) {
        let b = self.0.take().unwrap();
        let b = unsafe { ManuallyDrop::into_inner(b.Anonymous.Transition) };
        ManuallyDrop::into_inner(b.pResource);
    }
}

impl ResourceBarrier for TransitionBarrier {
    fn as_raw(&self) -> &D3D12_RESOURCE_BARRIER {
        self.0.as_ref().unwrap()
    }
}

impl std::fmt::Debug for TransitionBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.0.as_ref().unwrap();
        unsafe {
            write!(
                f,
                "TransitionBarrier {{ resource: {:?}, subresource: {:?}, state_before: {:?}, state_after: {:?} }}",
                b.Anonymous.Transition.pResource,
                b.Anonymous.Transition.Subresource,
                b.Anonymous.Transition.StateBefore,
                b.Anonymous.Transition.StateAfter
            )
        }
    }
}

#[derive(Clone)]
pub struct AliasingBarrier(Option<D3D12_RESOURCE_BARRIER>);

impl AliasingBarrier {
    #[inline]
    pub fn new(
        resource_before: &Resource,
        resource_after: &Resource,
        flags: D3D12_RESOURCE_BARRIER_FLAGS,
    ) -> Self {
        Self(Some(D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
            Flags: flags,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER {
                    pResourceBefore: ManuallyDrop::new(Some(resource_before.handle().clone())),
                    pResourceAfter: ManuallyDrop::new(Some(resource_after.handle().clone())),
                }),
            },
        }))
    }
}

impl Drop for AliasingBarrier {
    fn drop(&mut self) {
        let b = self.0.take().unwrap();
        let b = unsafe { ManuallyDrop::into_inner(b.Anonymous.Aliasing) };
        ManuallyDrop::into_inner(b.pResourceBefore);
        ManuallyDrop::into_inner(b.pResourceAfter);
    }
}

impl ResourceBarrier for AliasingBarrier {
    fn as_raw(&self) -> &D3D12_RESOURCE_BARRIER {
        self.0.as_ref().unwrap()
    }
}

impl std::fmt::Debug for AliasingBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.0.as_ref().unwrap();
        unsafe {
            write!(
                f,
                "AliasingBarrier {{ resource_before: {:?}, resource_after: {:?} }}",
                b.Anonymous.Aliasing.pResourceBefore, b.Anonymous.Aliasing.pResourceAfter
            )
        }
    }
}

#[derive(Clone)]
pub struct UavBarrier(Option<D3D12_RESOURCE_BARRIER>);

impl UavBarrier {
    #[inline]
    pub fn new(resource: &Resource, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> Self {
        Self(Some(D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_UAV,
            Flags: flags,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                UAV: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER {
                    pResource: ManuallyDrop::new(Some(resource.handle().clone())),
                }),
            },
        }))
    }
}

impl Drop for UavBarrier {
    fn drop(&mut self) {
        let b = self.0.take().unwrap();
        let b = unsafe { ManuallyDrop::into_inner(b.Anonymous.UAV) };
        ManuallyDrop::into_inner(b.pResource);
    }
}

impl ResourceBarrier for UavBarrier {
    fn as_raw(&self) -> &D3D12_RESOURCE_BARRIER {
        self.0.as_ref().unwrap()
    }
}

impl std::fmt::Debug for UavBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.0.as_ref().unwrap();
        unsafe {
            write!(
                f,
                "UavBarrier {{ resource: {:?} }}",
                b.Anonymous.UAV.pResource,
            )
        }
    }
}
