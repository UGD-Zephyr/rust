/* Programmer:          Per Stoor
 * Date:                2024-02-10
 * Last changed:        2024-02-10
 * Type of program:     Practicing enums...
 */

enum IpAddress {
    V4(IPV4),
    V6(IPV6),
}

struct IPV4 {

    public_ipv4:        (u8, u8, u8, u8),
    subnet_mask:        (u8, u8, u8, u8),
    local_ipv4_address: (u8, u8, u8, u8),
    router:             (u8, u8, u8, u8),

}

struct IPV6 {

    local_ipv6_address: String,

}

fn main(){ 

    let user_ipv4_address = IpAddress::V4(IPV4 {
        public_ipv4:        (10, 8, 32, 91),
        subnet_mask:        (255, 255, 255, 0),
        local_ipv4_address: (192,168,0,133),
        router:             (192, 168, 0, 1),
    });

    let user_ipv6_address = IpAddress::V6 (IPV6 {
        local_ipv6_address: String::from("fe80::4aba:4eff:fea7:6bc1"),
    });

        match user_ipv4_address {
            IpAddress::V4(ipv4) => {

                println!("public IP:            {:?}", ipv4.public_ipv4);
                println!("subnet mask:          {:?}", ipv4.subnet_mask);
                println!("local ipv4 address:   {:?}", ipv4.local_ipv4_address);
                println!("router:               {:?}", ipv4.router);
            }
            _ => println!("Unexpected IPV4 address!"),
        }

        match user_ipv6_address {
            IpAddress::V6(ipv6) => {
                println!("local ipv6 address:  {:?}", ipv6.local_ipv6_address);
            }
            _ => println!("Unexpected IPV6 address!"),
        }
    

} 
