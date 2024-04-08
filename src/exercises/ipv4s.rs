pub type IP = Vec<u8>;

pub fn ips_between(start: &str, end: &str) -> u32 {
    let start_ip = get_ip_octets(start);
    let end_ip = get_ip_octets(end);

    let octets_len = 4;
    assert!(start_ip.len() == octets_len);
    assert!(end_ip.len() == octets_len);

    calc_addr_val(&end_ip) - calc_addr_val(&start_ip)
}

fn calc_addr_val(ip: &IP) -> u32 {
    ip.iter()
        .rev()
        .enumerate()
        .map(|(exp, &v)| (v as u32) * 256u32.pow(exp as u32))
        .sum()
}

fn get_ip_octets(inp: &str) -> IP {
    inp.split('.').map(|v| v.parse::<u8>().unwrap()).collect()
}
