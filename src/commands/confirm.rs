use email_address::EmailAddress;

fn get_headers(encrypt: bool, sender: &EmailAddress, recipient: &EmailAddress) -> String {
	let common = format!("\
	From: {}\r\n\
	To: {}\r\n\
	Subject: Key publication confirmation\r\n\
	Wks-Draft-Version: 3\r\n", sender, recipient);
	
	let variable_headers = match encrypt {
		true => "Content-Type: multipart/encrypted; protocol=\"application/pgp-encrypted\"\r\n",
		false => "Content-Type: application/vnd.gnupg.wks\r\n"
	};

	format!("{}{}", common, variable_headers)
}

pub fn confirm_plaintext(user_email_addr: &EmailAddress, wks_email_addr: &EmailAddress, nonce: &String) -> Result<(), &'static str> {
	let headers = get_headers(false, user_email_addr, wks_email_addr);

	println!("\
	{}\r\n\
	type: confirmation-response\r\n\
	sender: {}\r\n\
	address: {}\r\n\
	nonce: {}", headers, wks_email_addr, user_email_addr, nonce);
	Ok(())
}

pub fn confirm_encrypt(user_email_addr: &EmailAddress, wks_email_addr: &EmailAddress, nonce: &String, key: &String) -> Result<(), &'static str> {
	let _headers = get_headers(true, user_email_addr, wks_email_addr);

	Err("Encrypted confirmation is not implemented. To not use encryption, omit the --key option.")
}
