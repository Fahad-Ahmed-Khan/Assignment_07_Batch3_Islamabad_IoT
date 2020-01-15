
//Calling piaic module's function here
fn main() {
    //Sub-Module Student's Functions
    piaic::student::submit_assingment();
    piaic::student::ask_question("Iot Stand for?");

    //Sub Module Teacher's Funtions
    piaic::teacher::mark_assignment();
    piaic::teacher::ans_question("IoT stand for Internet of Things");
}
