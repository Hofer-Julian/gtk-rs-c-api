pub(crate) fn builder() -> &'static gtk::Builder {
    unsafe {
        crate::BUILDER
            .as_ref()
            .expect("Builder instance should already be initialized.")
    }
}
