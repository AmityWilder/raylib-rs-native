pub const RLGL_VERSION: &'static str = "5.0";

/// Dynamic vertex buffers (position + texcoords + colors + indices arrays)
pub(crate) struct VertexBuffer {
    /// Number of elements in the buffer (QUADS)
    element_count: usize,

    /// Vertex position (XYZ - 3 components per vertex) (shader-location = 0)
    vertices: Vec<f32>,
    /// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    texcoords: Vec<f32>,
    /// Vertex normal (XYZ - 3 components per vertex) (shader-location = 2)
    normals: Vec<f32>,
    /// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
    colors: Vec<u8>,

    #[cfg(any(graphics_api_opengl_11, graphics_api_opengl_33))]
    /// Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    indices: Vec<u32>,

    #[cfg(graphics_api_opengl_es2)]
    /// Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    indices: Vec<u16>,

    /// OpenGL Vertex Array Object id
    vao_id: u32,
    /// OpenGL Vertex Buffer Objects id (5 types of vertex data)
    vbo_id: [u32; 5],
}

/// Draw call type
/// NOTE: Only texture changes register a new draw, other state-change-related elements are not
/// used at this moment (vaoId, shaderId, matrices), raylib just forces a batch draw call if any
/// of those state-change happens (this is done in core module)
pub(crate) struct DrawCall {
    /// Drawing mode: LINES, TRIANGLES, QUADS
    mode: usize,
    /// Number of vertex of the draw
    vertexCount: usize,
    /// Number of vertex required for index alignment (LINES, TRIANGLES)
    vertexAlignment: usize,
    /// Texture id to be used on the draw -> Use to create new draw call if changes
    textureId: u32,
}

/// rlRenderBatch type
pub(crate) struct RenderBatch {
    /// Number of vertex buffers (multi-buffering support)
    bufferCount: usize,
    /// Current buffer tracking in case of multi-buffering
    currentBuffer: usize,
    /// Dynamic buffer(s) for vertex data
    vertexBuffer: Vec<VertexBuffer>,

    /// Draw calls array, depends on textureId
    draws: Vec<DrawCall>,
    /// Draw calls counter
    drawCounter: usize,
    /// Current depth value for next draw
    currentDepth: f32,
}

// OpenGL version
pub enum GlVersion {
    /// OpenGL 1.1
    Gl11,
    /// OpenGL 2.1 (GLSL 120)
    Gl21,
    /// OpenGL 3.3 (GLSL 330)
    Gl33,
    /// OpenGL 4.3 (using GLSL 330)
    Gl43,
    /// OpenGL ES 2.0 (GLSL 100)
    GlES2_0,
    /// OpenGL ES 3.0 (GLSL 300 es)
    GlES3_0,
}

pub(crate) struct RLGL {

}

impl RLGL {
    // Initialize rlgl: OpenGL extensions, default buffers/shaders/textures, OpenGL states
    pub fn init(width: u32, height: u32) {
        // Enable OpenGL debug context if required
        if cfg!(rlgl_enable_opengl_debug_context) && cfg!(graphics_api_opengl_43) {
            if !glDebugMessageCallback.is_none() && !glDebugMessageControl.is_none() {
                glDebugMessageCallback(rlDebugMessageCallback, 0);
                // glDebugMessageControl(GL_DEBUG_SOURCE_API, GL_DEBUG_TYPE_ERROR, GL_DEBUG_SEVERITY_HIGH, 0, 0, GL_TRUE);

                // Debug context options:
                //  - GL_DEBUG_OUTPUT - Faster version but not useful for breakpoints
                //  - GL_DEBUG_OUTPUT_SYNCHRONUS - Callback is in sync with errors, so a breakpoint can be placed on the callback in order to get a stacktrace for the GL error
                glEnable(GL_DEBUG_OUTPUT);
                glEnable(GL_DEBUG_OUTPUT_SYNCHRONOUS);
            }
        }

        if cfg!(graphics_api_opengl_33) || cfg!(graphics_api_opengl_es2) {
            // Init default white texture
            unsigned char pixels[4] = { 255, 255, 255, 255 };   // 1 pixel RGBA (4 bytes)
            RLGL.State.defaultTextureId = rlLoadTexture(pixels, 1, 1, RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8, 1);

            if (RLGL.State.defaultTextureId != 0) TRACELOG(RL_LOG_INFO, "TEXTURE: [ID %i] Default texture loaded successfully", RLGL.State.defaultTextureId);
            else TRACELOG(RL_LOG_WARNING, "TEXTURE: Failed to load default texture");

            // Init default Shader (customized for GL 3.3 and ES2)
            // Loaded: RLGL.State.defaultShaderId + RLGL.State.defaultShaderLocs
            rlLoadShaderDefault();
            RLGL.State.currentShaderId = RLGL.State.defaultShaderId;
            RLGL.State.currentShaderLocs = RLGL.State.defaultShaderLocs;

            // Init default vertex arrays buffers
            // Simulate that the default shader has the location RL_SHADER_LOC_VERTEX_NORMAL to bind the normal buffer for the default render batch
            RLGL.State.currentShaderLocs[RL_SHADER_LOC_VERTEX_NORMAL] = RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL;
            RLGL.defaultBatch = rlLoadRenderBatch(RL_DEFAULT_BATCH_BUFFERS, RL_DEFAULT_BATCH_BUFFER_ELEMENTS);
            RLGL.State.currentShaderLocs[RL_SHADER_LOC_VERTEX_NORMAL] = -1;
            RLGL.currentBatch = &RLGL.defaultBatch;

            // Init stack matrices (emulating OpenGL 1.1)
            for (int i = 0; i < RL_MAX_MATRIX_STACK_SIZE; i++) RLGL.State.stack[i] = rlMatrixIdentity();

            // Init internal matrices
            RLGL.State.transform = rlMatrixIdentity();
            RLGL.State.projection = rlMatrixIdentity();
            RLGL.State.modelview = rlMatrixIdentity();
            RLGL.State.currentMatrix = &RLGL.State.modelview;
        }  // GRAPHICS_API_OPENGL_33 || GRAPHICS_API_OPENGL_ES2

        // Initialize OpenGL default states
        //----------------------------------------------------------
        // Init state: Depth test
        glDepthFunc(GL_LEQUAL);                                 // Type of depth testing to apply
        glDisable(GL_DEPTH_TEST);                               // Disable depth testing for 2D (only used for 3D)

        // Init state: Blending mode
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);      // Color blending function (how colors are mixed)
        glEnable(GL_BLEND);                                     // Enable color blending (required to work with transparencies)

        // Init state: Culling
        // NOTE: All shapes/models triangles are drawn CCW
        glCullFace(GL_BACK);                                    // Cull the back face (default)
        glFrontFace(GL_CCW);                                    // Front face are defined counter clockwise (default)
        glEnable(GL_CULL_FACE);                                 // Enable backface culling

        // Init state: Cubemap seamless
        if cfg!(graphics_api_opengl_33) {
            glEnable(GL_TEXTURE_CUBE_MAP_SEAMLESS);                 // Seamless cubemaps (not supported on OpenGL ES 2.0)
        }

        if cfg!(graphics_api_opengl_11) {
            // Init state: Color hints (deprecated in OpenGL 3.0+)
            glHint(GL_PERSPECTIVE_CORRECTION_HINT, GL_NICEST);      // Improve quality of color and texture coordinate interpolation
            glShadeModel(GL_SMOOTH);                                // Smooth shading between vertex (vertex colors interpolation)
        }

        if cfg!(graphics_api_opengl_33) || cfg!(graphics_api_opengl_es2) {
            // Store screen size into global variables
            RLGL.State.framebufferWidth = width;
            RLGL.State.framebufferHeight = height;

            TRACELOG(RL_LOG_INFO, "RLGL: Default OpenGL state initialized successfully");
            //----------------------------------------------------------
        }

        // Init state: Color/Depth buffers clear
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);                   // Set clear color (black)
        glClearDepth(1.0f);                                     // Set clear depth value (default)
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);     // Clear color and depth buffers (depth buffer required for 3D)
    }

    // Vertex Buffer Object deinitialization (memory free)
    pub fn rlglClose() {
        if cfg!(graphics_api_opengl_33) || cfg!(graphics_api_opengl_es2) {
            rlUnloadRenderBatch(RLGL.defaultBatch);

            // Unload default shader
            rlUnloadShaderDefault();

            // Unload default texture
            glDeleteTextures(1, &RLGL.State.defaultTextureId);
            TRACELOG(RL_LOG_INFO, "TEXTURE: [ID %i] Default texture unloaded successfully", RLGL.State.defaultTextureId);
        }
    }
}
