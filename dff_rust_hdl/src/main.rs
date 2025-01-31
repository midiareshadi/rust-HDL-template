use rust_hdl::prelude::*; 
use rand::{thread_rng, Rng};
use std::num::Wrapping;

const clock_freq_HZ : u64 = 100_000_000;

#[derive(LogicBlock, Default)]
struct dff_module {
    pub sig_d: Signal<In, Bit>,    
    pub clock: Signal<In, Clock>,
    pub sig_q: Signal<Out, Bit>,
    my_reg: DFF<Bit>,
}

impl Logic for dff_module {
  #[hdl_gen] 
  fn update(&mut self) {
       dff_setup!(self, clock, my_reg);
       self.my_reg.d.next = self.sig_d.val();
       self.sig_q.next = self.my_reg.q.val();
   }
}

fn main() {
  
        let test_cases = (0..1)
            .map(|_| {
                let d_val = thread_rng().gen::<bool>();
            })
            .collect::<Vec<_>>();

        let mut sim = simple_sim!(dff_module, clock, clock_freq_HZ, ep, {
            let mut x = ep.init()?; 
            wait_clock_cycle!(ep, clock, x);            
            ep.done(x)
        });


   sim.run_to_file(
    dff_module::default().into(),
    10 * sim_time::ONE_MILLISECOND,
    &vcd_path!("dff_VCD.vcd"),
)
.unwrap();

}
