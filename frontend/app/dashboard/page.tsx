'use client';
import React, { useEffect, useState } from 'react';

interface Organization {
  id: string;
  name: string;
  description: string;
  slogan: string;
}

export default function Dashboard() {
  const [orgs, setOrgs] = useState<Organization[]>([]);
  const [liveVotes, setLiveVotes] = useState<Record<string, number>>({});

  useEffect(() => {
    // Fetch organizations
    fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/organizations`)
      .then(res => res.json())
      .then(data => setOrgs(data));

    // Connect WebSocket
    const socket = new WebSocket('ws://localhost:8080/ws/');
    socket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'VOTE_UPDATE') {
        setLiveVotes(prev => ({
          ...prev,
          [data.candidate_id]: data.votes
        }));
      }
    };

    return () => socket.close();
  }, []);

  return (
    <div style={{ display: 'flex', minHeight: '100vh', fontFamily: 'sans-serif' }}>
      {/* Sidebar */}
      <nav style={{ width: '250px', background: '#0047AB', color: 'white', padding: '2rem' }}>
        <h2>UON-EOMS</h2>
        <ul style={{ listStyle: 'none', padding: 0 }}>
          <li style={{ padding: '10px 0' }}>Organizations</li>
          <li style={{ padding: '10px 0' }}>Active Elections</li>
          <li style={{ padding: '10px 0' }}>Profile</li>
        </ul>
      </nav>

      {/* Main Content */}
      <main style={{ flex: 1, padding: '2rem', background: '#f4f4f4' }}>
        <h1>Student Organizations</h1>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))', gap: '1rem' }}>
          {orgs.map(org => (
            <div key={org.id} style={{ background: 'white', padding: '1.5rem', borderRadius: '8px', boxShadow: '0 2px 4px rgba(0,0,0,0.1)' }}>
              <h3 style={{ color: '#0047AB', marginTop: 0 }}>{org.name}</h3>
              <p style={{ fontStyle: 'italic', color: '#666' }}>"{org.slogan}"</p>
              <p>{org.description}</p>
              <button 
                onClick={() => window.location.href = `/elections/${org.id}`}
                style={{ background: '#0047AB', color: 'white', border: 'none', padding: '8px 16px', borderRadius: '4px', cursor: 'pointer' }}
              >
                View Elections
              </button>
            </div>
          ))}
        </div>
      </main>
    </div>
  );
}
