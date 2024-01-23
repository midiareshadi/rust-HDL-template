use rust_hdl::prelude::*;   
use rand::{thread_rng, Rng};
use std::num::Wrapping;

const CLOCK_FREQ_HZ : u64 = 100_000_000;

#[derive(LogicBlock, Default)]
struct Adder8b {
    pub sig_a: Signal<In, Bits<8>>,
    pub sig_b: Signal<In, Bits<8>>,
    pub sig_sum: Signal<Out, Bits<8>>,
    pub clock: Signal<In, Clock>,
    my_reg: DFF<Bits<8>>,
}

impl Logic for Adder8b {
  #[hdl_gen]  
  fn update(&mut self) {
       
       dff_setup!(self, clock, my_reg);
       self.my_reg.d.next = self.sig_a.val() + self.sig_b.val();
       self.sig_sum.next = self.my_reg.q.val();
   }
}

fn main() {
        let test_cases = (0..512)
            .map(|_| {
                let a_val = thread_rng().gen::<u8>();
                let b_val = thread_rng().gen::<u8>();
                let c_val = (Wrapping(a_val) + Wrapping(b_val)).0;
                (
                    a_val.to_bits::<8>(),
                    b_val.to_bits::<8>(),
                    c_val.to_bits::<8>(),
                )
            })
            .collect::<Vec<_>>();
        
        let mut sim = simple_sim!(Adder8b, clock, CLOCK_FREQ_HZ, ep, {
            let mut x = ep.init()?; 
            for test_case in &test_cases {
                x.sig_a.next = test_case.0;
                x.sig_b.next = test_case.1;
                
                wait_clock_cycle!(ep, clock, x);
                
                println!(
                    "Test case {:x} + {:x} = {:x} (check {:x})",
                    test_case.0,
                    test_case.1,
                    x.sig_sum.val(),
                    test_case.2
                );
                
                sim_assert_eq!(ep, x.sig_sum.val(), test_case.2, x);
            }
            
            ep.done(x)
        });
        // Simple running simulation
        // sim.run(Adder8b::default().into(), sim_time::ONE_MILLISECOND)
            // .unwrap();

        sim.run_to_file(
            Adder8b::default().into(),
            sim_time::ONE_MILLISECOND,
            "sims/VCD_adder.vcd",
        )
        .unwrap();

        vcd_to_svg(
            "sims/VCD_adder.vcd",
            "images/SVG_adder.svg",
            &[
                "uut.clock",
                "uut.sig_a",
                "uut.sig_b",
                "uut.my_reg.d",
                "uut.my_reg.q",
                "uut.sig_sum",
            ],
            0,
            100 * sim_time::ONE_NANOSECOND,
        )
        .unwrap()
}