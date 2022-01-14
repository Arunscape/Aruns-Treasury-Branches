import { createClient } from '@supabase/supabase-js'

const supabaseUrl = 'https://bkiuhzngtkoliiqqqunp.supabase.co';
const supabaseKey = process.env.SUPABASE_KEY ?? "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTY0MDMwMTkyNCwiZXhwIjoxOTU1ODc3OTI0fQ.vl7VuWJSjhOXRB0ufzMwuzMIg3HKmsHzELJZZpE87EQ";
const supabase = createClient(supabaseUrl, supabaseKey);

const unix_seconds = () => Math.floor(Date.now() / 1000);


export {
    unix_seconds,
    supabase,
}