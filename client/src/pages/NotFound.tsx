import React from 'react'
import { Link } from 'react-router-dom'
export default function NotFound() {
    return (
        <div>
            <h2>This is the Not found page</h2>
            <Link to="/">Home page</Link>
        </div>
    )
}