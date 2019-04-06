use nx;

pub struct LibraryApplet
{
    holder: nx::AppletHolder,
}

impl LibraryApplet
{
    pub fn new(id: nx::AppletId, mode: nx::LibAppletMode, version: u32) -> nx::HorizonResult<Self>
    {
        unsafe
        {
            let mut aph: nx::AppletHolder = std::mem::zeroed();
            let mut rc = nx::appletCreateLibraryApplet(&mut aph, id, mode);
            resultok!(rc, { println!("Exit"); });
            let mut largs: nx::LibAppletArgs = std::mem::zeroed();
            nx::libappletArgsCreate(&mut largs, version);
            rc = nx::libappletArgsPush(&mut largs, &mut aph);
            resultfinal!(rc, LibraryApplet { holder: aph })
        }
    }

    pub fn push_data(&mut self, data: *const u8, size: usize) -> nx::HorizonResult<()>
    {
        unsafe
        {
            let rc = nx::libappletPushInData(&mut self.holder, data as *const nx::c::c_void, size);
            resultfinal!(rc)
        }
    }

    pub fn show(&mut self) -> nx::HorizonResult<()>
    {
        unsafe
        {
            let rc = nx::appletHolderStart(&mut self.holder);
            resultfinal!(rc)
        }
    }

    pub fn show_and_wait(&mut self) -> nx::HorizonResult<()>
    {
        unsafe
        {
            let rc = nx::appletHolderStart(&mut self.holder);
            resultok!(rc);
            while nx::appletHolderWaitInteractiveOut(&mut self.holder)
            {

            }
            nx::appletHolderJoin(&mut self.holder);
            resultfinal!(rc)
        }
    }

    pub fn pop_data(&mut self, out: *mut u8, size: usize) -> nx::HorizonResult<usize>
    {
        unsafe
        {
            let mut tsize: usize = 0;
            let rc = nx::libappletPopOutData(&mut self.holder, out as *mut nx::c::c_void, size, &mut tsize);
            resultfinal!(rc, tsize)
        }
    }
}

impl Drop for LibraryApplet
{
    fn drop(&mut self)
    {
        unsafe
        {
            nx::appletHolderClose(&mut self.holder);
        }
    }
}