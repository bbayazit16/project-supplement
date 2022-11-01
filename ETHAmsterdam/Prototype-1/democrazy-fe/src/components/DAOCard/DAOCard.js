import * as React from 'react';
import Box from '@mui/material/Box';
import Card from '@mui/material/Card';
import CardActions from '@mui/material/CardActions';
import CardContent from '@mui/material/CardContent';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';
import {Avatar, Link, Stack} from "@mui/material";
import "./DAOCard.css";
import {Link as RouterLink} from "react-router-dom";

export default function DAOCard(props) {
  const {name, id, img_url, description} = props.dao;
  return (
    // <Card sx={{ minWidth: 275 }}>
    //   <CardContent>
    //     <Stack direction="row" spacing={2} className={"dao-title-container"}>
    //       <Avatar
    //         alt={name}
    //         src={img_url}
    //         sx={{ width: 56, height: 56 }}
    //       />
    //       <Typography variant="h5" component="div">
    //         {name}
    //       </Typography>
    //     </Stack>


    //     <Typography variant="body2">
    //       {description}

    //     </Typography>
    //   </CardContent>
    //   <CardActions>
    //     <Button size="small"><Link component={RouterLink} to={"dao/"+id}>View</Link></Button>
    //   </CardActions>
    // </Card>

    <Stack alignItems="center" sx={{
      border: "1.5px solid #fff", 
      borderRadius: 5,
      padding: "20px 0px",
      }}>
      <Avatar
        alt={name}
        src={img_url}
        sx={{ width:150, height: 150}}
      />
      <Typography variant="h5" component="div" sx={{fontWeight: 900, marginTop: 2}}>
        {name}
      </Typography>
      <Typography variant="body2" textAlign="center" paddingX="30px">
        {description}
      </Typography>
      <Button size="medium" sx={{marginTop: 1, border: "1.5px solid #fff"}}><Link component={RouterLink} to={"dao/"+id} sx={{textDecoration: "none"}}>View</Link></Button>
    </Stack>
  );
}
