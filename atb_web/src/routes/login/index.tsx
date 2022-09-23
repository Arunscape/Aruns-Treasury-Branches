import { component$, useContext, useContextProvider } from '@builder.io/qwik';
import { DocumentHead, RequestHandler } from '@builder.io/qwik-city';
import { useLocation } from '@builder.io/qwik-city';
import { AuthContext } from '~/util/auth';


export const onGet: RequestHandler<any> = async ({ request, response }) => {
    const authcontext = useContext(AuthContext);
    // above line errors out
    if (!authcontext?.token) {
      throw response.redirect('/');
    }
  };

export default component$(() => {

    const location = useLocation();

    const authcontext = useContext(AuthContext)

    if (location?.query?.token){
        authcontext.token = location.query.token
    }


    console.log("Authcontext: ", authcontext)    
        
    return <>
    {
        (!location?.query?.token || !authcontext?.token) ? 
            <p>Connect to the server in Minecraft mc.arun.gg and type <em>/atb login</em> to get a login URL</p>
            : <p>Logging in...</p>
    }
        </>
    
});


export const head: DocumentHead = {
    title: 'Login',
  };