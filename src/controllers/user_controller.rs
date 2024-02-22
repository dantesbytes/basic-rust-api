fn handle_post_request(request: &str) -> (String, String) {

    /*

    function attempts to extract user information
    from the request, establish a database connection,
    and insert the user data into the database.
    If successful, it returns a success response; 
    otherwise, it returns an error response.

     */

    match(get_user_request_body(&request), Client::connect(DB_URL, NoTls)) {

        (Ok(user), Ok(mut Client)) => {
            
            Client
            
            .execute(
                
                "INSERT INTO users (name, email Values ($1, $2)",
                &[&user.name, &user.email]
            )
            .unwrap();
        (OK_RESPONSE.to_string(), "User created".to_string())

        }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),

    }
    /*

     */
}
    


fn handle_get_request(request: &str) -> (String, String) {

    /*
    function attempts to extract an ID from the 
    request, establish a database connection, 
    and retrieve user data based on the ID from
     the database. If successful, it returns a 
     success response containing the retrieved 
     user data in JSON format; otherwise, it returns an
      appropriate error response.
    */


    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {

        // if both are ok
        (Ok(id), Ok(mut client)) => 

        match client.query_one("SELECT * FROM users Where id = $1", &[&id]){

            Ok(row) => {

                let user = User {

                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),

                };

                (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
            }

            _ => (NOT_FOUND.to_string(), "User not found".to_string()),
        
        }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }

}

fn handle_get_all_request(request: &str) -> (String, String) {

    match Client::connect(DB_URL, NoTls) {

        Ok(mut client) => {

            let mut users = Vec::new();

            for row in client.query("SELECT * FROM USERS", &[]).unwrap() {

                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),

                });
            }

                (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())


            }

            _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        }
    }



fn handle_put_request(request: &str) -> (String, String) {

    match 
    (
        get_id(&request).parse::<i32>(),
        get_user_request_body(&request),
        Client::connect(DB_URL, NoTls),


    )
    {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
            .execute(
                "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                &[&user.name, &user.email, &id]
            )
            .unwrap();

        (OK_RESPONSE.to_string(), "user updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "ERROR".to_string()),
    }

}

fn handle_delete_request(request: &str) {


}

