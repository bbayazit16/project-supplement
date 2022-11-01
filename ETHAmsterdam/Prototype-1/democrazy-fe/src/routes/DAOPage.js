import { Avatar, Divider, Grid, Link, Stack, Typography } from "@mui/material";
import ProposalCard from "../components/ProposalCard";
import { Outlet } from "@mui/icons-material";
import { Link as RouterLink, useParams } from "react-router-dom";
import Button from "@mui/material/Button";

export function DAOPage({ daos, proposals }) {
  const params = useParams();
  const dao = daos.find(e => e.id === params.daoId);
  const props = [];
  for (const proposal of proposals) {
    if (proposal.daoId === dao.id) {
      props.push(proposal);
    }
  }

  if (!dao) {
    return <p>No dao found with given id</p>;
  }

  return (
    <Grid container spacing={2}>
      <Grid item lg={8} xs={12}>
        <div>
          <Typography sx={{fontWeight: 200, marginBottom: 5, ml: "50px" }} variant={"h4"}>Proposals</Typography>
          <Stack spacing={2}>
            {proposals.map(proposal => (
              <ProposalCard key={proposal.id} proposal={proposal} />
            ))}
          </Stack>
        </div>
      </Grid>

      <Grid item lg={4} xs={12} sx={{ position: "fixed", right: 0, bottom: 0, mr: 10, mb: 5, width: "300px"}}>
        <Stack
          sx={{ mb: 2 }}
          direction="row"
          spacing={2}
          className={"dao-title-container"}
        >
          <Avatar
            alt={dao.name}
            src={dao.img_url}
            sx={{ width: 56, height: 56 }}
          />
          <Typography variant="h4" component="h4" sx={{fontWeight: 200}}>
            {dao.name}
          </Typography>
        </Stack>
        {" "}
        <Divider variant="middle" />
        <Typography sx={{ my: 2 }} variant="body1">
          {dao.description}
        </Typography>
        <Button size="large" variant="contained" color="button" sx={{ textDecoration: "none", mt: 3 }}>
          <Link component={RouterLink} to={"create"}>
            Create a proposal
          </Link>
        </Button>
      </Grid>
    </Grid>
  );
}

export function DAOContainer() {
  console.log("Dao container page");
  return <Outlet />;
}
