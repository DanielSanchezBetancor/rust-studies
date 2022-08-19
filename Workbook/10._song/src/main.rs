fn main() {
    //Variable initialization
    //Days that presents were brought
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twefth" ];
    //Present that will be given
    let presents_list = ["All their good wishes", "Gifts for one and all", "Some mistletoe", "A guardian angel", "Gold and silver tinsel", "Candles a glowing", "Little silver bells", "A shining star", "Four colored lights", "Three boughs of holly", "Two candy canes"];
    //Counter of how many presents we got (since I dont know yet how to access array length)
    let mut present_counter = 11;
    //Loop - For each day, we say every line in the chorus
    for day in days {
        println!("On the {day} day of Christmas");
        println!("My good friends brought to me");
        //loop to access all available presents
        for pos in present_counter..11 {
            let present = presents_list[pos];
            println!("({present})");
        }
        let final_verse = if present_counter == 11 { "A song and a Christmas tree" } else { "And a song for the Christmas tree" };
        println!("{final_verse}");
        println!("");
        present_counter = if present_counter > 0 { present_counter - 1 } else { 0 };
    }
}
