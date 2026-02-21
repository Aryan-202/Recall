import React, { useState } from 'react';
import { Mail, Lock, Layers, ArrowRight, Loader2 } from 'lucide-react';
import { motion } from 'framer-motion';
import './Login.css';

interface LoginProps {
    onLogin: (email: string) => void;
}

const Login: React.FC<LoginProps> = ({ onLogin }) => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setIsLoading(true);
        setError(null);

        // Mock login since we don't have the auth command yet
        // In a real app, this would call a Tauri command like `invoke("login", { email, password })`
        setTimeout(() => {
            if (email && password) {
                onLogin(email);
            } else {
                setError('Please enter both email and password');
                setIsLoading(false);
            }
        }, 1500);
    };

    return (
        <div className="login-page">
            <motion.div
                className="login-card"
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5 }}
            >
                <div className="login-header">
                    <div className="login-logo">
                        <Layers size={32} className="accent-color" />
                    </div>
                    <h1>Welcome Back</h1>
                    <p>Sign in to your Recall account</p>
                </div>

                <form className="login-form" onSubmit={handleSubmit}>
                    {error && <div className="login-error">{error}</div>}

                    <div className="input-field">
                        <label htmlFor="email">Email Address</label>
                        <div className="input-wrapper">
                            <Mail size={18} className="input-icon" />
                            <input
                                id="email"
                                type="email"
                                placeholder="name@example.com"
                                value={email}
                                onChange={(e) => setEmail(e.target.value)}
                                required
                            />
                        </div>
                    </div>

                    <div className="input-field">
                        <label htmlFor="password">Password</label>
                        <div className="input-wrapper">
                            <Lock size={18} className="input-icon" />
                            <input
                                id="password"
                                type="password"
                                placeholder="••••••••"
                                value={password}
                                onChange={(e) => setPassword(e.target.value)}
                                required
                            />
                        </div>
                    </div>

                    <button className="login-submit-btn" type="submit" disabled={isLoading}>
                        {isLoading ? (
                            <Loader2 size={18} className="animate-spin" />
                        ) : (
                            <>
                                <span>Sign In</span>
                                <ArrowRight size={18} />
                            </>
                        )}
                    </button>
                </form>

                <div className="login-footer">
                    <p>Don't have an account? <a href="#">Create one</a></p>
                </div>
            </motion.div>

            <div className="login-background">
                <div className="blob blob-1"></div>
                <div className="blob blob-2"></div>
            </div>
        </div>
    );
};

export default Login;
