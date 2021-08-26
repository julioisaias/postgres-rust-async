use tokio_postgres::{ NoTls, Error };

mod models;

#[tokio::main]
async fn main() -> Result<(), Error> {


    let (client, connection) = tokio_postgres::connect("postgresql://postgres:r00t@localhost:5432/sis11dev", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });


    let rows = client.query("SELECT * FROM public.user", &[]).await?;

    for row in &rows{
        let user = models::User { 
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            email: row.get(3),
            password: row.get(4),
            mobilephone: row.get(5),
            creation_date: row.get(6),
            last_update: row.get(7),
            is_active: row.get(8),
        };

        println!("{},{},{},{},{},{},{},{},{}", 
            user.id, 
            user.first_name, 
            user.last_name, 
            user.email, 
            user.password, 
            user.mobilephone,
            user.creation_date(),
            user.last_update(),
            user.is_active
        );

        

    }
    
    Ok(())
}