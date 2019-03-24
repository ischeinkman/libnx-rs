use nx;

pub struct LibraryApplet
{
    holder: nx::AppletHolder,
    args: nx::LibAppletArgs,
}

impl LibraryApplet
{
    pub unsafe fn new(id: nx::AppletId, mode: nx::LibAppletMode, version: u32) -> Result<Self, u32>
    {
        let mut aph : nx::AppletHolder = core::mem::zeroed();
        let mut rc = nx::appletCreateLibraryApplet(&mut aph, id, mode);
        resultok!(rc);
        let mut largs : nx::LibAppletArgs = core::mem::zeroed();
        nx::libappletArgsCreate(&mut largs, version);
        rc = nx::libappletArgsPush(&mut largs, &mut aph);
        resultfinal!(rc, LibraryApplet { holder: aph, args: largs })
    }

    pub unsafe fn push_data(&mut self, data: *const u8, size: usize) -> Result<(), u32>
    {
        let rc = nx::libappletPushInData(&mut self.holder, data as *const nx::lang_items::c_void, size);
        resultfinal!(rc)
    }

    pub unsafe fn show(&mut self) -> Result<(), u32>
    {
        let rc = nx::appletHolderStart(&mut self.holder);
        resultfinal!(rc)
    }

    pub unsafe fn show_and_wait(&mut self) -> Result<(), u32>
    {
        let rc = nx::appletHolderStart(&mut self.holder);
        resultok!(rc);
        while nx::appletHolderWaitInteractiveOut(&mut self.holder)
        {

        }
        nx::appletHolderJoin(&mut self.holder);
        resultfinal!(rc)
    }

    pub unsafe fn pop_data(&mut self, out: *mut u8, size: usize) -> Result<usize, u32>
    {
        let mut tsize : usize = 0;
        let rc = nx::libappletPopOutData(&mut self.holder, out as *mut nx::lang_items::c_void, size, &mut tsize);
        resultfinal!(rc, tsize);
    }
}