const API_URL = `http://localhost:${process.env.REACT_APP_API_PORT}/api`

type GroupName = {
    key: string
    display_name: string
}

export async function getGroupNames(): Promise<GroupName[]> {
    return await fetch(`${API_URL}/groups`)
        .then(res => res.json())
        .catch(console.log);
}

type ChallengeName = {
    key: string
    display_name: string
}

type GroupInfo = {
    name: string
    url: string
    challenges: ChallengeName[]
}

export type ChallengeInfo = {
    title: string
    description: string
}

export async function getGroupInfo(key: string): Promise<GroupInfo> {
    return await fetch(`${API_URL}/groups/${key}`)
        .then(res => res.json())
        .catch(console.log);
}

export async function getChallengeInfo(groupKey: string, challengeKey: string): Promise<ChallengeInfo> {
    return await fetch(`${API_URL}/groups/${groupKey}/${challengeKey}`)
        .then(res => res.json())
        .catch(console.log);
}

export async function solveChallenge(groupKey: string, challengeKey: string, data: string): Promise<string> {
    return await fetch(`${API_URL}/groups/${groupKey}/${challengeKey}/solve`, {
        method: 'POST',
        body: data,
    })
        .then(res => res.text());
}