#[derive(Debug)]
struct Movie{
    title:String,
    release_Year:u32,
    length_in_minutes: u32
}

fn main() {
        let Movie_1=Movie{
            title:String::from("Lord Of the Rings"),
            release_Year:2001,
            length_in_minutes:228
        };
        println!("Complete Movie Details");
        println!("{:#?}",Movie_1 );

        println!("\n\n\nDetails using each Field");
        println!("\nTitle:{}",Movie_1.title );
        println!("Release Date:{}",Movie_1.release_Year );
        println!("Length (in minutes):{}",Movie_1.length_in_minutes );
       
       
       
        // creating another using of Movie using the fields of Movie_1
        let Movie_2=Movie{
            title:Movie_1.title,
            release_Year:Movie_1.release_Year,
            length_in_minutes:Movie_1.length_in_minutes
        };

       
       
       
        //Returning Movie Instance and printing it.
        println!("\n\n\nMovie Created From User Defined Function ");
        let Movie_3=create_Movie("The Hobbit".to_string(), 2012, 182);
        println!("\n{:#?}",Movie_3 );


}

fn create_Movie(title:String,release_Year:u32,length_in_minutes:u32) -> Movie{
        Movie{
            title,
            release_Year,
            length_in_minutes
        }
}
