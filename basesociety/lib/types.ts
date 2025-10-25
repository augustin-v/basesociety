export interface AgentProfile {
  name: string;
  personality: string;
  desires: string;
  skills: string[];
}

export interface AgentDetails {
  agent_id: string;  // Snake_case from backend
  profile: AgentProfile;
}

export interface CustomMessage {
  role: string;  // 'user' | 'assistant' | 'system'
  content: string;
  origin: 'Agent' | 'Owner' | 'System';
  timestamp: string;  // ISO
}