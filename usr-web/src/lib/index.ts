// place files you want to import through the `$lib` alias in this folder.
export type Team = 'Software' | 'Mechanical' | 'Electrical' | 'Systems' | 'Admin' | 'Social';

const TEAM_SET = new Set(['Software', 'Mechanical', 'Electrical', 'Systems', 'Admin', 'Social']);

export class TeamQuery {
	private constructor(
		private left: string | TeamQuery,
		private right: string | TeamQuery,
		private operator: 'and' | 'or'
	) {}

	public static parse(query: string): TeamQuery {
		query = query.replaceAll('(', ' ( ').replaceAll(')', ' ) ');
		// Replace all '!' with trailing spaces (any number) with just '!'
		query = query.replaceAll(/! +/g, '!');

		// Check for '!' preceeding '(' or ')' and throw an error
		if (query.includes('!(') || query.includes('!)')) {
			throw new Error("Invalid '!' placement. It can only be used before a name.");
		}

		const tokens = query.split(' ').filter((token) => token.length > 0);
		if (tokens.length === 1) {
			tokens.push('and');
			tokens.push(tokens[0]);
		}
		return TeamQuery.parseTokens(tokens, 0, tokens.length);
	}

	private static parseTokens(tokens: string[], start: number, end: number): TeamQuery {
		let left: string | TeamQuery = tokens[start];
		let nextIndex;

		if (left === '(') {
			nextIndex = TeamQuery.parenthesesRange(tokens, start + 1);
			left = TeamQuery.parseTokens(tokens, start + 1, nextIndex);
			// Skip over closing parantheses
			nextIndex++;
			if (nextIndex >= end) {
				return left;
			}
		} else {
			nextIndex = start + 1;
		}

		if (tokens[nextIndex] === undefined) {
			throw new Error('Invalid query');
		}
		const operator = tokens[nextIndex].toLowerCase();
		nextIndex++;
		if (operator !== 'and' && operator !== 'or') {
			throw new Error('Invalid operator: ' + operator);
		}

		let right: string | TeamQuery = tokens[nextIndex];
		let nextNextIndex;
		if (right === '(') {
			nextNextIndex = TeamQuery.parenthesesRange(tokens, nextIndex + 1);
			right = TeamQuery.parseTokens(tokens, nextIndex + 1, nextNextIndex);
			// Skip over closing parantheses
			nextNextIndex++;
		} else {
			nextNextIndex = nextIndex + 1;
		}
		if (nextNextIndex >= end) {
			return new TeamQuery(left, right, operator);
		}

		if (tokens[nextNextIndex] === undefined) {
			throw new Error('Invalid query');
		}
		const nextOperator = tokens[nextNextIndex].toLowerCase();
		nextNextIndex++;
		if (nextOperator !== 'and' && nextOperator !== 'or') {
			throw new Error('Invalid operator: ' + nextOperator);
		}

		if (tokens[nextNextIndex] === '(') {
			const nextQuery = TeamQuery.parseTokens(
				tokens,
				nextNextIndex + 1,
				TeamQuery.parenthesesRange(tokens, nextNextIndex + 1)
			);
			return new TeamQuery(new TeamQuery(left, right, operator), nextQuery, nextOperator);
		} else {
			return new TeamQuery(
				new TeamQuery(left, right, operator),
				tokens[nextNextIndex],
				nextOperator
			);
		}
	}

	private static parenthesesRange(tokens: string[], start: number): number {
		let depth = 0;
		let i = start;
		while (depth >= 0) {
			if (tokens[i] === undefined) {
				throw new Error('Unmatched parentheses');
			}
			if (tokens[i] == '(') {
				depth++;
			}
			if (tokens[i] == ')') {
				depth--;
			}
			i++;
		}
		return i - 1;
	}

	public verifyNames(names: Set<string>): string | null {
		let leftValid;
		if (typeof this.left === 'string') {
			let word = this.left;
			if (word.startsWith('!')) {
				word = word.substring(1);
			}
			leftValid = word === '*' || names.has(word) || TEAM_SET.has(word) ? null : word;
		} else {
			leftValid = this.left.verifyNames(names);
		}
		if (leftValid !== null) {
			return leftValid;
		}
		if (typeof this.right === 'string') {
			let word = this.right;
			if (word.startsWith('!')) {
				word = word.substring(1);
			}
			return word === '*' || names.has(word) || TEAM_SET.has(word) ? null : word;
		} else {
			return this.right.verifyNames(names);
		}
	}

	private static stringToSet(
		token: string,
		teams: Record<string, string[]>,
		names: Set<string>
	): Set<string> | string {
		if (token.startsWith('!')) {
			let negationSet = TeamQuery.stringToSet(token.substring(1), teams, names);
			if (typeof negationSet === 'string') {
				if (names.has(negationSet)) {
					return new Set();
				} else {
					return names;
				}
			} else {
				return names.difference(negationSet);
			}
		} else if (token === '*') {
			return names;
		} else if (TEAM_SET.has(token)) {
			return new Set(teams[token] ?? []);
		}

		return token;
	}

	public evaluate(teams: Record<string, string[]>, names: Set<string>): Set<string> {
		let leftSet;
		let rightSet;

		if (typeof this.left === 'string' && typeof this.right === 'string') {
			leftSet = TeamQuery.stringToSet(this.left, teams, names);
			rightSet = TeamQuery.stringToSet(this.right, teams, names);
		} else if (typeof this.left !== 'string' && typeof this.right !== 'string') {
			leftSet = this.left.evaluate(teams, names);
			rightSet = this.right.evaluate(teams, names);
		} else {
			if (typeof this.left === 'string') {
				leftSet = TeamQuery.stringToSet(this.left, teams, names);
				rightSet = (this.right as TeamQuery).evaluate(teams, names);
			} else {
				leftSet = this.left.evaluate(teams, names);
				rightSet = TeamQuery.stringToSet(this.right as string, teams, names);
			}
		}

		if (this.operator === 'and') {
			if (typeof leftSet === 'string') {
				if (typeof rightSet === 'string') {
					if (names.has(leftSet) && names.has(rightSet)) {
						return new Set([leftSet, rightSet]);
					} else {
						return new Set();
					}
				} else {
					if (names.has(leftSet)) {
						return rightSet.add(leftSet);
					} else {
						return new Set();
					}
				}
			} else if (typeof rightSet === 'string') {
				if (names.has(rightSet)) {
					return leftSet.add(rightSet);
				} else {
					return new Set();
				}
			} else {
				return leftSet.intersection(rightSet);
			}
		} else {
			if (typeof leftSet === 'string') {
				if (typeof rightSet === 'string') {
					const out = new Set<string>();
					if (names.has(leftSet)) {
						out.add(leftSet);
					}
					if (names.has(rightSet)) {
						out.add(rightSet);
					}
					return out;
				} else {
					if (names.has(leftSet)) {
						return rightSet.add(leftSet);
					} else {
						return rightSet;
					}
				}
			} else if (typeof rightSet === 'string') {
				if (names.has(rightSet)) {
					return leftSet.add(rightSet);
				} else {
					return leftSet;
				}
			} else {
				return leftSet.union(rightSet);
			}
		}
	}
}
