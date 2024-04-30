// create a type for person
type Person = {
    name: string;
    age: number;
};

// create a function that takes a person and returns a greeting

const greet = (person: Person): string => {
    return `Hello ${person.name}`;
};

// create a person object
const person = {
    name: "John",
    age: 30,
};

// call the greet function with the person object and log the result
console.log("person is cool...");
console.log(greet(person));
