use super::*;
use std::mem::ManuallyDrop;
use windows::Win32::Graphics::Direct3D12::*;

pub trait ResourceBarrier {
    fn as_raw(&self) -> &D3D12_RESOURCE_BARRIER;
}

#[derive(Clone)]
pub struct TransitionBarrier(D3D12_RESOURCE_BARRIER);

impl TransitionBarrier {
    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER::default()),
            },
            ..Default::default()
        })
    }

    #[inline]
    pub fn resource(mut self, resource: &Resource) -> Self {
        unsafe {
            let barrier = D3D12_RESOURCE_BARRIER {
                Type: self.0.Type,
                Flags: self.0.Flags,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                        pResource: ManuallyDrop::new(Some(resource.handle().clone())),
                        Subresource: self.0.Anonymous.Transition.Subresource,
                        StateBefore: self.0.Anonymous.Transition.StateBefore,
                        StateAfter: self.0.Anonymous.Transition.StateAfter,
                    }),
                },
            };
            ManuallyDrop::drop(&mut (*self.0.Anonymous.Transition).pResource);
            ManuallyDrop::drop(&mut self.0.Anonymous.Transition);
            Self(barrier)
        }
    }

    #[inline]
    pub fn subresource(mut self, subresource: u32) -> Self {
        unsafe {
            (*self.0.Anonymous.Transition).Subresource = subresource;
            self
        }
    }

    #[inline]
    pub fn state_before(mut self, state: D3D12_RESOURCE_STATES) -> Self {
        unsafe {
            (*self.0.Anonymous.Transition).StateBefore = state;
            self
        }
    }

    #[inline]
    pub fn state_after(mut self, state: D3D12_RESOURCE_STATES) -> Self {
        unsafe {
            (*self.0.Anonymous.Transition).StateAfter = state;
            self
        }
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> Self {
        self.0.Flags = flags;
        self
    }
}

impl Drop for TransitionBarrier {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut (*self.0.Anonymous.Transition).pResource);
            ManuallyDrop::drop(&mut self.0.Anonymous.Transition);
        }
    }
}

impl ResourceBarrier for TransitionBarrier {
    fn as_raw(&self) -> &D3D12_RESOURCE_BARRIER {
        &self.0
    }
}

impl std::fmt::Debug for TransitionBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = &self.0;
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
pub struct AliasingBarrier(D3D12_RESOURCE_BARRIER);

impl AliasingBarrier {
    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER::default()),
            },
        })
    }

    #[inline]
    pub fn resource_before(mut self, resource: &Resource) -> Self {
        unsafe {
            let barrier = D3D12_RESOURCE_BARRIER {
                Type: self.0.Type,
                Flags: self.0.Flags,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER {
                        pResourceBefore: ManuallyDrop::new(Some(resource.handle().clone())),
                        pResourceAfter: ManuallyDrop::new(ManuallyDrop::take(
                            &mut (*self.0.Anonymous.Aliasing).pResourceAfter,
                        )),
                    }),
                },
            };
            ManuallyDrop::drop(&mut (*self.0.Anonymous.Aliasing).pResourceBefore);
            ManuallyDrop::drop(&mut self.0.Anonymous.Aliasing);
            Self(barrier)
        }
    }

    #[inline]
    pub fn resource_after(mut self, resource: &Resource) -> Self {
        unsafe {
            let barrier = D3D12_RESOURCE_BARRIER {
                Type: self.0.Type,
                Flags: self.0.Flags,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER {
                        pResourceBefore: ManuallyDrop::new(ManuallyDrop::take(
                            &mut (*self.0.Anonymous.Aliasing).pResourceBefore,
                        )),
                        pResourceAfter: ManuallyDrop::new(Some(resource.handle().clone())),
                    }),
                },
            };
            ManuallyDrop::drop(&mut (*self.0.Anonymous.Aliasing).pResourceAfter);
            ManuallyDrop::drop(&mut self.0.Anonymous.Aliasing);
            Self(barrier)
        }
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> Self {
        self.0.Flags = flags;
        self
    }
}

impl Drop for AliasingBarrier {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut (*self.0.Anonymous.Aliasing).pResourceBefore);
            ManuallyDrop::drop(&mut (*self.0.Anonymous.Aliasing).pResourceAfter);
            ManuallyDrop::drop(&mut self.0.Anonymous.Aliasing);
        }
    }
}

impl ResourceBarrier for AliasingBarrier {
    fn as_raw(&self) -> &D3D12_RESOURCE_BARRIER {
        &self.0
    }
}

impl std::fmt::Debug for AliasingBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = &self.0;
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
pub struct UavBarrier(D3D12_RESOURCE_BARRIER);

impl UavBarrier {
    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_UAV,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                UAV: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER::default()),
            },
        })
    }

    #[inline]
    pub fn resource(mut self, resource: &Resource) -> Self {
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: self.0.Type,
            Flags: self.0.Flags,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                UAV: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER {
                    pResource: ManuallyDrop::new(Some(resource.handle().clone())),
                }),
            },
        };
        unsafe {
            ManuallyDrop::drop(&mut (*self.0.Anonymous.UAV).pResource);
            ManuallyDrop::drop(&mut self.0.Anonymous.UAV);
        }
        Self(barrier)
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> Self {
        self.0.Flags = flags;
        self
    }
}

impl Drop for UavBarrier {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut (*self.0.Anonymous.UAV).pResource);
            ManuallyDrop::drop(&mut self.0.Anonymous.UAV);
        }
    }
}

impl ResourceBarrier for UavBarrier {
    fn as_raw(&self) -> &D3D12_RESOURCE_BARRIER {
        &self.0
    }
}

impl std::fmt::Debug for UavBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = &self.0;
        unsafe {
            write!(
                f,
                "UavBarrier {{ resource: {:?} }}",
                b.Anonymous.UAV.pResource,
            )
        }
    }
}
