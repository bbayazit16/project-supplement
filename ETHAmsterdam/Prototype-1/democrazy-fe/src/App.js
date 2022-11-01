import * as React from "react";
import PropTypes from "prop-types";
import Box from "@mui/material/Box";
import CssBaseline from "@mui/material/CssBaseline";
import Drawer from "@mui/material/Drawer";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemText from "@mui/material/ListItemText";
import Toolbar from "@mui/material/Toolbar";
import { Home } from "./routes/Home";
import { ProposalPage } from "./routes/ProposalPage";
import {
  BrowserRouter,
  Link as RouterLink,
  Route,
  Routes,
} from "react-router-dom";
import { ConnectWalletComponent } from "./components/ConnectWalletComponent";

import { Web3ReactProvider } from "@web3-react/core";
import { Web3Provider } from "@ethersproject/providers";
import WalletConnectProvider from "@walletconnect/web3-provider";
import {DAOPage} from "./routes/DAOPage";
import title from "./media/democrazy.png"
import {ProposalCreate} from "./routes/ProposalCreate";
import {QueryClient, QueryClientProvider, } from 'react-query';
import {blueGrey} from '@mui/material/colors';
import { maxHeight } from '@mui/system';

const queryClient = new QueryClient();
function getLibrary() {
  const p = new WalletConnectProvider({
    rpc: {
      69: process.env.ALCHEMY_URL,
    },
  });

  return new Web3Provider(p);
}

const drawerWidth = 240;

function App(props) {
  const { window } = props;
  const [mobileOpen, setMobileOpen] = React.useState(false);

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  const [daos, setDaos] = React.useState([]);
  const [proposals, setProposals] = React.useState([]);

  React.useEffect(() => {
    fetch("https://dc-backend-rpal.vercel.app/dao").then(daosb => {
      daosb.json().then(daos => {
        const ids = daos.map(dao => dao.id);
        const proposals = [];
        ids.forEach(id => {
          fetch(`https://dc-backend-rpal.vercel.app/dao/${id}/proposals`).then(
            proposalstream => {
              proposalstream.json().then(proposal => {
                for (const e of proposal) {
                  proposals.push(e);
                }
              });
            }
          );
        });
        setDaos(daos);
        setProposals(proposals);
      });
    });
  }, []);

  const drawer = (
    <Box display="flex" alignItems="center" justifyContent="center">
      <Toolbar />
      <List style={{
        textAlign: "center"
      }}>
        <ListItem to={"/"} component={RouterLink} key={'Home'}>
            <img
              src={title}
              alt="logo"
              width={170}
              style={{
                marginLeft: -50,
                marginTop: 15
              }}
            />
        </ListItem>
      </List>
      <Box style={{
        position: "fixed",
        top: 120,
        width: 200,
        height: "60%",
        borderRadius: 15,
        backgroundColor: blueGrey[900],
      }}>
        {/* <h1>What the fuck</h1> */}
      </Box>
      <List style={{
        position: "fixed",
        bottom: 0,
        textAlign: "center",
        paddingBottom: 10,
      }} >

        <ListItem>
          <ListItemText>
            <ConnectWalletComponent />
          </ListItemText>
        </ListItem>
      </List>
    </Box>
  );

  const container =
    window !== undefined ? () => window().document.body : undefined;

  return (
    <Web3ReactProvider getLibrary={getLibrary}>
      <BrowserRouter>
        <QueryClientProvider client={queryClient}>
          <Box sx={{ display: "flex" }}>
            <CssBaseline />
            {/* <AppBar
        position="fixed"
        sx={{
          width: {sm: `calc(100% - ${drawerWidth}px)`},
          ml: {sm: `${drawerWidth}px`},
        }}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{mr: 2, display: {sm: 'none'}}}
          >
            <MenuIcon/>
          </IconButton>
          <Typography variant="h6" noWrap component="div">
            democrazy
          </Typography>
        </Toolbar>
      </AppBar> */}
            <Box
              component="nav"
              sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
              aria-label="menu"
            >
              <Drawer
                variant="permanent"
                sx={{
                  display: { xs: "none", sm: "block" },
                  "& .MuiDrawer-paper": {
                    boxSizing: "border-box",
                    width: drawerWidth,
                  },
                }}
                open
              >
                {drawer}
              </Drawer>
            </Box>
            <Box
              component="main"
              sx={{
                flexGrow: 1,
                p: 3,
                width: { sm: `calc(100% - ${drawerWidth}px)` },
              }}
            >
              <Toolbar />

              <Routes>
                <Route path="/" element={<Home daos={daos} />} />
                <Route
                  path="dao/:daoId/proposal/:proposalId"
                  element={<ProposalPage proposals={proposals} daos={daos} />}
                />
                <Route
                  path={"dao/:daoId"}
                  element={<DAOPage daos={daos} proposals={proposals} />}
                />
                <Route
                  path={"dao/:daoId/create/"}
                  element={<ProposalCreate />}
                />
                <Route
                  path="*"
                  element={
                    <main style={{ padding: "1rem" }}>
                      <p>There's nothing here!</p>
                    </main>
                  }
                />
              </Routes>
            </Box>
          </Box>
        </QueryClientProvider>
      </BrowserRouter>
    </Web3ReactProvider>
  );
}

App.propTypes = {
  /**
   * Injected by the documentation to work in an iframe.
   * You won't need it on your project.
   */
  window: PropTypes.func,
};

export default App;
