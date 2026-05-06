import React from 'react';

export default function Home() {
  return (
    <main style={{ fontFamily: 'sans-serif', padding: '2rem', maxWidth: '800px', margin: '0 auto' }}>
      <h1 style={{ color: '#0047AB' }}>UON Electoral & Organization Management System (UON-EOMS)</h1>
      <p>Welcome to the official University of Nairobi student organization election platform.</p>
      
      <div style={{ marginTop: '2rem', display: 'flex', gap: '1rem' }}>
        <a href="/dashboard" style={{ padding: '0.5rem 1rem', background: '#0047AB', color: 'white', textDecoration: 'none', borderRadius: '4px' }}>
          Login via Student Portal
        </a>
      </div>
      
      <section style={{ marginTop: '3rem' }}>
        <h2>Live Elections</h2>
        <div style={{ padding: '1rem', border: '1px solid #ccc', borderRadius: '4px', background: '#f9f9f9' }}>
          <h3>ONUSS - Chairperson</h3>
          <p>Voting is currently: <strong>OPEN</strong></p>
        </div>
      </section>
    </main>
  );
}
