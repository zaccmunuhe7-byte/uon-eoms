'use client';
import React, { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';

interface Position {
  id: string;
  name: string;
  description: string;
}

interface Candidate {
  id: string;
  user_id: string;
  position_id: string;
  manifesto: string;
  votes_count: number;
}

export default function ElectionsPage() {
  const { orgId } = useParams();
  const [positions, setPositions] = useState<Position[]>([]);
  const [candidates, setCandidates] = useState<Record<string, Candidate[]>>({});
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const posRes = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/organizations/${orgId}/positions`);
        const posData = await posRes.json();
        setPositions(posData);

        const candMap: Record<string, Candidate[]> = {};
        for (const pos of posData) {
          const candRes = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/positions/${pos.id}/candidates`);
          candMap[pos.id] = await candRes.json();
        }
        setCandidates(candMap);
      } catch (err) {
        console.error("Failed to fetch election data", err);
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, [orgId]);

  const handleVote = async (posId: string, candId: string) => {
    const token = localStorage.getItem('token');
    if (!token) {
      alert("Please login to vote");
      window.location.href = '/login';
      return;
    }

    const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/vote`, {
      method: 'POST',
      headers: { 
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({ position_id: posId, candidate_id: candId }),
    });

    if (res.ok) {
      alert("Vote cast successfully!");
      window.location.reload();
    } else {
      const msg = await res.text();
      alert(`Error: ${msg}`);
    }
  };

  if (loading) return <div style={{ padding: '2rem' }}>Loading election data...</div>;

  return (
    <div style={{ maxWidth: '900px', margin: '0 auto', padding: '2rem', fontFamily: 'sans-serif' }}>
      <header style={{ marginBottom: '2rem', borderBottom: '2px solid #0047AB', paddingBottom: '1rem' }}>
        <h1 style={{ color: '#0047AB' }}>Active Elections</h1>
        <button onClick={() => window.location.href = '/dashboard'} style={{ background: 'none', border: '1px solid #0047AB', color: '#0047AB', padding: '5px 10px', borderRadius: '4px', cursor: 'pointer' }}>
          ← Back to Dashboard
        </button>
      </header>

      {positions.length === 0 && <p>No active positions for this organization.</p>}

      {positions.map(pos => (
        <section key={pos.id} style={{ marginBottom: '3rem', background: 'white', padding: '1.5rem', borderRadius: '8px', boxShadow: '0 2px 8px rgba(0,0,0,0.1)' }}>
          <h2 style={{ color: '#333' }}>{pos.name}</h2>
          <p style={{ color: '#666' }}>{pos.description}</p>
          
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '1rem', marginTop: '1rem' }}>
            {candidates[pos.id]?.map(cand => (
              <div key={cand.id} style={{ border: '1px solid #eee', padding: '1rem', borderRadius: '8px', textAlign: 'center' }}>
                <div style={{ width: '80px', height: '80px', background: '#eee', borderRadius: '50%', margin: '0 auto 1rem' }}></div>
                <h4 style={{ margin: '0 0 0.5rem' }}>Candidate {cand.id.slice(0, 5)}</h4>
                <p style={{ fontSize: '0.9rem', color: '#555', minHeight: '3rem' }}>{cand.manifesto || "No manifesto provided."}</p>
                <div style={{ fontWeight: 'bold', fontSize: '1.2rem', margin: '1rem 0', color: '#0047AB' }}>
                  {cand.votes_count} Votes
                </div>
                <button 
                  onClick={() => handleVote(pos.id, cand.id)}
                  style={{ width: '100%', padding: '10px', background: '#28a745', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer', fontWeight: 'bold' }}
                >
                  Vote for Candidate
                </button>
              </div>
            ))}
          </div>
        </section>
      ))}
    </div>
  );
}
