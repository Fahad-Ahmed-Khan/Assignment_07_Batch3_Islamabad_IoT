pub mod student
    {
        pub fn submit_assingment()
        {
            println!("Dear Student you assignment is submitted successfully.")
        }

        pub fn ask_question(question:&str)
        {
            println!("Your question '{}' is posted. Our team will response you back asap.",question)
        }
    }
    pub mod teacher
    {
        pub fn mark_assignment()
        {
            println!("Assigment is marked successfully.")
        }

        pub fn ans_question(ans:&str)
        {
            println!("Your answer is submitted. '{}'",ans);
        }
    }