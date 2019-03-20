pub mod screenmanager {

    pub struct screenmanager {
        current_screen : screen;
        screens : Vec<screen>;
    }

    impl screenmanager {
        
        fn init_screen() {

        }
        
        fn render_screen() {
            
        }

        fn update_screen() {
            
        }
    }
}

trait screen {
    fn init(&mut self);
    fn render(&mut self, args: &RenderArgs);
    fn update(&mut self, args: &UpdateArgs);
}
