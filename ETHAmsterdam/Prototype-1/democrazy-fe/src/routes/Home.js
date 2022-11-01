import Typography from "@mui/material/Typography";
import DAOCard from "../components/DAOCard/DAOCard";
import {Grid} from "@mui/material";
//  const {name, id, img_url, description} = props.dao;
export function Home({daos}) {
  return (<>
      <Grid container spacing={3}>
        {
          daos.map((e)=>(<Grid md={4} xs={12} key={e.id} item><DAOCard dao={e} /></Grid>))
        }
      </Grid>
  </>)
}
