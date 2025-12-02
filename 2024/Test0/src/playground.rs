
fn mut_s(r0: &mut String, r1: &String) -> () {

}

pub fn play() {
    let mut s = String::from("hello");

    {
        let mut r1 = &mut s;
        let mut r2 = &mut s;
        //r1 = &mut s;
        //*r1 = s;
        mut_s(&mut r1, &r2);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
