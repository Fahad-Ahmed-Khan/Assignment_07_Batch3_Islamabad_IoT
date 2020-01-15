//Using lib.rs as module here
mod lib;

fn main() {
    //Sub-Module Student's Functions
    lib::student::submit_assingment();
    lib::student::ask_question("Iot Stand for?");

    //Sub Module Teacher's Funtions
    lib::teacher::mark_assignment();
    lib::teacher::ans_question("IoT stand for Internet of Things");
}
