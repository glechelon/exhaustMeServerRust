pub struct Credentials{
    login: String,
    password: String
}

pub struct Account{
    credentials: Credentials,
    key: String,
    user_id: u64 //TODO: remplacer par un UUID
}

//TODO: Il faut un POST pour la création du compte

//TODO: Il faut un 