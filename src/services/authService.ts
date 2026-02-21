import { invoke } from "@tauri-apps/api/core";
import { User } from "../types";

export interface AuthResponse {
    user: User;
    access_token: string;
}

export const authService = {
    async login(email: string, password: string): Promise<User> {
        try {
            const response = await invoke<AuthResponse>("login", { email, password });
            return response.user;
        } catch (error) {
            console.error("Login service error:", error);
            throw error;
        }
    },

    async signup(email: string, password: string): Promise<User> {
        try {
            const response = await invoke<AuthResponse>("signup", { email, password });
            return response.user;
        } catch (error) {
            console.error("Signup service error:", error);
            throw error;
        }
    }
};
