//
// This is only a SKELETON file for the 'Phone Number' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const clean = (phone_number) => {
	if (/[a-z]/.test(phone_number)) {
		throw new Error("Letters not permitted");
	}

	if (/[@!:]/.test(phone_number)) {
		throw new Error("Punctuations not permitted");
	}

	phone_number = phone_number.replace(/\D/g, "");

	console.log(phone_number);

	let area_code = "";
	let exchange_code = "";
	let subscriber_number = "";

	if (phone_number.split("").length === 11) {
		if (phone_number.startsWith("1")) {
			const phone_num_arr = phone_number.split("");
			// extract area code, exchange code and subscriber number from a 11 digit number
			area_code = phone_num_arr.slice(1, 4).join("");
			exchange_code = phone_num_arr.slice(4, 7).join("");
			subscriber_number = phone_num_arr.slice(7, 11).join("");
		} else {
			throw new Error("11 digits must start with 1");
		}
	} else if (phone_number.split("").length === 10) {
		const phone_num_arr = phone_number.split("");
		// extract area code, exchange code and subscriber number from a 10 digit number
		area_code = phone_num_arr.slice(0, 3).join("");
		exchange_code = phone_num_arr.slice(3, 6).join("");
		subscriber_number = phone_num_arr.slice(6, 10).join("");
	} else if (phone_number.split("").length < 10) {
		throw new Error("Incorrect number of digits");
	} else {
		throw new Error("More than 11 digits");
	}

	if (area_code.startsWith("0")) {
		throw new Error("Area code cannot start with zero");
	}

	if (area_code.startsWith("1")) {
		throw new Error("Area code cannot start with one");
	}

	if (exchange_code.startsWith("0")) {
		throw new Error("Exchange code cannot start with zero");
	}

	if (exchange_code.startsWith("1")) {
		throw new Error("Exchange code cannot start with one");
	}

	return phone_number;
};
