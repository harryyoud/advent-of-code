use itertools::Itertools;
use aoc_2020::get_input;

fn main() {
    let input = get_input(13);
    let mut input = input.lines();

    let arrival_time = input.next().unwrap().parse::<u32>().unwrap();
    let bus_schedules = input.next().unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, bus_id)| *bus_id != "x")
        .map(|(x, bus_id)| (x, bus_id.parse::<u32>().unwrap()))
        .collect_vec();
    let part_1 = bus_schedules
        .iter()
        .map(|(_, bus_id)| (bus_id, bus_id - (arrival_time % bus_id)))
        .min_by_key(|(_bus_id, wait_time)| *wait_time)
        .map(|(bus_id, wait_time)| bus_id * wait_time)
        .unwrap();

    dbg!(part_1);
}

// Runs too long, there is probably a more optimal solution
// Strategy: step by largest bus cycle, and check modulo for other buses (after changing the time t
//           by the offset in list)
fn part_2(bus_schedules: &[(usize, u32)]) -> u64 {
    let longest_cycle = bus_schedules.iter().max_by_key(|(_, bus_id)| *bus_id).unwrap();

    let mut bus_schedules = bus_schedules.into_iter()
        .map(|(idx, bus_id)| (*idx as i64 - longest_cycle.0 as i64, bus_id))
        .collect_vec();
    bus_schedules.sort_by_key(|x| *x.1);

    let mut t = longest_cycle.1 as u64;
    loop {
        if bus_schedules.iter().all(|(offset, bus_id)| t.saturating_add_signed(*offset) % **bus_id as u64 == 0) {
            return t.saturating_add_signed(bus_schedules[0].0);
        }

        t += longest_cycle.1 as u64;
    }
}