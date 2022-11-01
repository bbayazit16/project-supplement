import express = require("express")
import config = require("../configs/config.json")
import firebase = require("firebase/firestore")
import firebaseapp = require("firebase/app")

const app = firebaseapp.initializeApp(config.firebaseConfig)
const db = firebase.getFirestore(app)

const server = express()

//#region view
server.get("/dao/:id", async (req, res) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let daoInfo: any = await firebase.getDoc(
        firebase.doc(db, "daos", req.params.id)
    )

    daoInfo = { ...daoInfo.data(), id: daoInfo.id }

    return res.status(200).json(daoInfo)
})

server.get("/dao", async (req, res) => {
    const querySnapshot = await firebase.getDocs(
        firebase.collection(db, "daos")
    )

    const allEntries: firebase.QueryDocumentSnapshot[] = []

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    querySnapshot.forEach(doc =>
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        allEntries.push({ ...doc.data(), id: doc.id } as any)
    )

    return res.status(200).json(allEntries)
})

server.get("/dao/:id/proposals", async (req, res) => {
    const querySnapshot = await firebase.getDocs(
        firebase.collection(db, "proposals")
    )

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const allEntries: any = []

    querySnapshot.forEach(doc => allEntries.push({ ...doc.data(), id: doc.id }))

    const filteredProposals = allEntries.filter(
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (proposal: any) => proposal.daoId == req.params.id
    )
    // querySnapshot.forEach(doc => allEntries.push({ ... doc.data(), id: doc.id}))
    return res.status(200).json(filteredProposals)
})

server.get("/dao/:id/proposals/:proposalId", async (req, res) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let proposalInfo: any = await firebase.getDoc(
        firebase.doc(db, "proposals", req.params.proposalId)
    )
    proposalInfo = { ...proposalInfo.data(), id: proposalInfo.id }
    return res.status(200).json(proposalInfo)
})

//#endregion

//#region post



//#endregion

server.listen(config.port, () => {
    console.log("Democrazy listening on port", config.port)
})
