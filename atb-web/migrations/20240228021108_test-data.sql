insert into users (id) values ('a9363265-23c3-4b30-8483-b2b91050cdab'),  ('da3ed9e8-75bc-4bef-af9a-00943a3c219d');

insert into accounts (id, userid, nickname) values
  ('b211cd62-73d5-433e-8cd8-5349ea35abde', 'a9363265-23c3-4b30-8483-b2b91050cdab', 'testuser1nickname1'),
  ('481b122c-199a-4578-9c32-a0cb115ff30c', 'da3ed9e8-75bc-4bef-af9a-00943a3c219d', 'testuser2nickname1');

insert into mc_items (id) values ('diamond');

insert into transactions (fromid, toid, quantity, item, price) values
  ('b211cd62-73d5-433e-8cd8-5349ea35abde', '481b122c-199a-4578-9c32-a0cb115ff30c', 64, 'diamond', 100),
  ('481b122c-199a-4578-9c32-a0cb115ff30c', 'b211cd62-73d5-433e-8cd8-5349ea35abde', 64, 'diamond', 100);
