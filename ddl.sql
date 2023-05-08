create table nfts (
  id                   SERIAL PRIMARY KEY,
  name                 VARCHAR(256) NOT NULL,
  contract_address     VARCHAR(256) NOT NULL,
  network              VARCHAR(256) NOT NULL,
  discord_url          VARCHAR(256) NOT NULL
);

create table addresses (
  id                   SERIAL PRIMARY KEY,
  name                 VARCHAR(256) NOT NULL,
  address              VARCHAR(256) NOT NULL,
  network              VARCHAR(256) NOT NULL,
  discord_url          VARCHAR(256) NOT NULL
);
