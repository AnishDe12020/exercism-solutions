//
// This is only a SKELETON file for the 'High Scores' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export class HighScores {
	constructor() {
		this.scores = scores;
	}

	get scores() {
		return this.scores;
	}

	get latest() {
		return this.scores[0];
	}

	get personalBest() {
		return Math.max(this.scores);
	}

	get personalTopThree() {
		throw new Error("Remove this statement and implement this function");
	}
}
