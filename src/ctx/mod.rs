// region:    --- Modules


// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64,
}

// Constructor.
impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx { user_id: 0 }
	}

	pub fn new(user_id: i64) -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self { user_id })
	}
}

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}
}