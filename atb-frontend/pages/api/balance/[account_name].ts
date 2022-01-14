// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import { supabase } from '../../../utils/db'



const handler = async (
  req: NextApiRequest,
  // res: NextApiResponse<Data>
  res: NextApiResponse<Account>,
) => {

  const account_name = req.query;
  const r = await supabase.from('accounts').select("*");

  res.status(200).json(r)
}

export default handler;