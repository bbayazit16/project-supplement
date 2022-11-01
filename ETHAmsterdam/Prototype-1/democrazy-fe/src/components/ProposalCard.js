import * as React from "react";
import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import FaceIcon from "@mui/icons-material/Face";
import {Chip, Link, Stack} from "@mui/material";
import CalendarMonthIcon from "@mui/icons-material/CalendarMonth";
import {Link as RouterLink} from "react-router-dom";

export default function ProposalCard({ proposal }) {
  const props = proposal.status?.date.seconds;
  const lbl = new Date(proposal?.date);
  console.log(lbl)
  return (
    <Box sx={{ minWidth: 200 }}>
      <Card
        variant="confined"
        sx={{
          borderRadius: 5,
          border: "1px solid white",
          backgroundColor: "background.default",
          paddingX: "8px",
          paddingBottom: "10px",
          width: "600px",
          marginLeft: "50px"
        }}
      >
        <React.Fragment>
          <CardContent>
            <Typography sx={{ fontSize: 14 }} color="text.primary" gutterBottom>
              By{" "}
              <Chip
                variant="outlined"
                color="primary"
                size="small"
                icon={<FaceIcon />}
                label={proposal.author.substring(0, 10) + "..."}
                component={"span"}
              />
            </Typography>
            <Typography sx={{ fontWeight: 900, marginTop: 1.5 }} variant="h5">
              {proposal.title}
            </Typography>

            <Typography variant="body2">
              {proposal.description.substring(0, 140)}
              {proposal.description.length > 140 ? "..." : ""}
            </Typography>
            <Typography sx={{ mb: 1.5 }} color="text.secondary">
              Status
            </Typography>
            <Stack
              direction={"row"}
              spacing={1}
              className={"dao-title-container"}
            >
              {proposal.status.result !== undefined && (
                <Chip
                  sx={{ height: 35, borderRadius: 50 }}
                  color={proposal.status.result ? "success" : "error"}
                  label={proposal.status?.result ? "Aye" : "Nay"}
                  component={"span"}
                />
              )}
              <Chip
                sx={{ height: 35, paddingX: 1 }}
                variant="outlined"
                color={
                  lbl > Date.now()
                    ? "secondary"
                    : "error"
                }
                size="small"
                icon={<CalendarMonthIcon />}
                label={lbl.toString()}
                component={"span"}
              />
            </Stack>
          </CardContent>
          <CardActions>
            <Link component={RouterLink} to={"proposal/" + proposal.id}>
              <Button size="large" variant="contained" color="button" sx={{ textDecoration: "none" }}>
                  Learn More
              </Button>
            </Link>
          </CardActions>
        </React.Fragment>
      </Card>
    </Box>
  );
}
