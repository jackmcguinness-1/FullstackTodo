interface GLoginStatus {
    loggedIn: boolean,
    user: GUserDetails | null,
}

interface GUserDetails {
    name: string,
    email: string,
    picture: string,
}