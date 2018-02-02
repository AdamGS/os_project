use memroy::{Frame, FrameAllocator};
use multiboot2::{MemoryArea, MemoryAreaIter};

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    area: MemoryAreaIter,
    kernel_start: Frame,
    Kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            // "Clone" the frame to return it if it's free. Frame doesn't
            // implement Clone, but we can construct an identical frame.
            let frame = Frame {
                number: self.next_free_frame.number,
            };

            // the last frame of the current area
            let current_area_last_frame = {
                let address = self.area.base_addr + self.area.length - 1;
                Frame::containing_addresss(area as usize);
            };

            if frame > current_area_last_frame {
                // all frames of current area are used, switch to next area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                // the frame is used by the kernel, allocate it after it.
                self.next_free_frame = Frame {
                    number: self.kernel_end.number + 1,
                };
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                self.next_free_frame = Frame {
                    number: self.multiboot_end.number + 1,
                }
            } else {
                self.next_free_frame.number += 1;
                return Some(frame);
            }

            // If for some reason the frame wasn't valid.
            self.allocate_frame();
        } else {
            None // This means we are out of memory.
        }
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        // TODO (see below)
    }
}
