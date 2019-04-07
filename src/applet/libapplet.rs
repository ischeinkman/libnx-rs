use sys;
use os;

pub struct LibraryApplet
{
    holder: sys::AppletHolder,
}

impl LibraryApplet
{
    pub fn new(id: sys::AppletId, mode: sys::LibAppletMode, version: u32) -> os::Result<Self>
    {
        unsafe
        {
            let mut aph: sys::AppletHolder = std::mem::zeroed();
            let mut rc = sys::appletCreateLibraryApplet(&mut aph, id, mode);
            result_assert!(rc, { println!("Exit"); });
            let mut largs: sys::LibAppletArgs = std::mem::zeroed();
            sys::libappletArgsCreate(&mut largs, version);
            rc = sys::libappletArgsPush(&mut largs, &mut aph);
            result_final!(rc, LibraryApplet { holder: aph })
        }
    }

    pub fn push_data(&mut self, data: *const u8, size: usize) -> os::Result<()>
    {
        unsafe
        {
            let rc = sys::libappletPushInData(&mut self.holder, data as *const std::ffi::c_void, size);
            result_final!(rc)
        }
    }

    pub fn show(&mut self) -> os::Result<()>
    {
        unsafe
        {
            let rc = sys::appletHolderStart(&mut self.holder);
            result_final!(rc)
        }
    }

    pub fn show_and_wait(&mut self) -> os::Result<()>
    {
        unsafe
        {
            let rc = sys::appletHolderStart(&mut self.holder);
            result_assert!(rc);
            while sys::appletHolderWaitInteractiveOut(&mut self.holder)
            {

            }
            sys::appletHolderJoin(&mut self.holder);
            result_final!(rc)
        }
    }

    pub fn pop_data(&mut self, out: *mut u8, size: usize) -> os::Result<usize>
    {
        unsafe
        {
            let mut tsize: usize = 0;
            let rc = sys::libappletPopOutData(&mut self.holder, out as *mut std::ffi::c_void, size, &mut tsize);
            result_final!(rc, tsize)
        }
    }
}

impl Drop for LibraryApplet
{
    fn drop(&mut self)
    {
        unsafe
        {
            sys::appletHolderClose(&mut self.holder);
        }
    }
}