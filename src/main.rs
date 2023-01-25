
pub fn gen_can_ips(pad: u8, can: u8) -> Vec<String> {
    let mut ip_list = Vec::new();
    for x in 1..256 {
        let ip = format!("10.{}.{}.{}", pad, ((can-1)*2), x);
        ip_list.push(ip);
    }
    for x in 1..256 {
        let ip = format!("10.{}.{}.{}", pad, (((can-1)*2) + 1), x);
        ip_list.push(ip);
    }
    ip_list
}


// call this function let ips = assign_ips_to_cans(3);
//*******************************************************/
// Need to add the 20 and 36 pad
//********************************************************/
fn assign_ips_to_cans(can_number: u8) -> Vec<String> {
    
    //24 pad ip vector
    if can_number <= 24{
        return gen_can_ips(24, can_number)
    }

    // 20 pad ip vector
    else if can_number <= 44{
        return gen_can_ips(20, (can_number-24))
    }
    //36 pad
    else {
        return gen_can_ips(24, (can_number-44))
    }
}



// just the 24 pad vector create
fn assign_ips_to_24_pad() -> Vec<String> {
    let mut ip_list = Vec::new();
    let mut temp = Vec::new();

    for x in 1..25{
        temp = gen_can_ips(24, x);
        ip_list.append(&mut temp);
    }   

    ip_list
}

//20 pad ip vector
fn assign_ips_to_20_pad() -> Vec<String> {
    let mut ip_list = Vec::new();
    let mut temp = Vec::new();

    for x in 1..21{
        temp = gen_can_ips(20, x);
        ip_list.append(&mut temp);
    } 
    
    ip_list
}

//36 pad vector
fn assign_ips_to_36_pad() -> Vec<String> {
    let mut ip_list = Vec::new();
    let mut temp = Vec::new();

    for x in 1..37{
        temp = gen_can_ips(36, x);
        ip_list.append(&mut temp);
    }     

    ip_list
}

//site ip vector
 
fn assign_ips_to_site() -> Vec<String> {
    let mut ip_list = Vec::new();
    let mut temp = Vec::new();
    
    temp = assign_ips_to_24_pad();
    ip_list.append(&mut temp);
    temp = assign_ips_to_20_pad();
    ip_list.append(&mut temp);
    temp = assign_ips_to_36_pad();
    ip_list.append(&mut temp);

    ip_list
}





fn main() {
    //let ips = assign_ips_to_cans(1);
    //let ips = generate_ips();
    let ips = assign_ips_to_site();


    for i in ips {
        println!("{}", i);
    }

    //testing
    let ip = "10.24.2.24"
    let miner = client.get_miner(&ip, None).await;
}
