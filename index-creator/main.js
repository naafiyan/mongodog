// read index_map file from root of project dir
const data = require("./data/index_map.json");

// TODO: N - read from env vars/cli args but okay to hardcode for now :)
let db = connect("mongodb://localhost:27017/socials");

// read json from file
printjson(data);

// map through all the objects in the hashmap and create index in mongodb
Object.entries(data).map(([collection_name, index_field_name]) => {
	db.runCommand(
		{
			createIndexes: collection_name,
			indexes: [
				{
					key: {
						index_field_name: 1
					},
					name: index_field_name,
				},
			]
		}
	)
	console.log("Index created on field: " + index_field_name + " in collection " + collection_name);
});

console.log(db.users.getIndexes())

