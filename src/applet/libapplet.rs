use native;
use os;

pub struct LibraryApplet
{
    holder: native::AppletHolder,
}

impl LibraryApplet
{
    pub fn new(id: native::AppletId, mode: native::LibAppletMode, version: u32) -> os::Result<Self>
    {
        unsafe
        {
            let mut aph: native::AppletHolder = std::mem::zeroed();
            let mut rc = native::appletCreateLibraryApplet(&mut aph, id, mode);
            result_assert!(rc, { println!("Exit"); });
            let mut largs: native::LibAppletArgs = std::mem::zeroed();
            native::libappletArgsCreate(&mut largs, version);
            rc = native::libappletArgsPush(&mut largs, &mut aph);
            result_final!(rc, LibraryApplet { holder: aph })
        }
    }

    pub fn push_data(&mut self, data: *const u8, size: usize) -> os::Result<()>
    {
        unsafe
        {
            let rc = native::libappletPushInData(&mut self.holder, data as *const std::ffi::c_void, size);
            result_final!(rc)
        }
    }

    pub fn show(&mut self) -> os::Result<()>
    {
        unsafe
        {
            let rc = native::appletHolderStart(&mut self.holder);
            result_final!(rc)
        }
    }

    pub fn show_and_wait(&mut self) -> os::Result<()>
    {
        unsafe
        {
            let rc = native::appletHolderStart(&mut self.holder);
            result_assert!(rc);
            while native::appletHolderWaitInteractiveOut(&mut self.holder)
            {

            }
            native::appletHolderJoin(&mut self.holder);
            result_final!(rc)
        }
    }

    pub fn pop_data(&mut self, out: *mut u8, size: usize) -> os::Result<usize>
    {
        unsafe
        {
            let mut tsize: usize = 0;
            let rc = native::libappletPopOutData(&mut self.holder, out as *mut std::ffi::c_void, size, &mut tsize);
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
            native::appletHolderClose(&mut self.holder);
        }
    }
}